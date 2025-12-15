use id3::{Tag, TagLike, Version};
use pulldown_cmark::{Event, Parser, Tag as MdTag};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

type MusicFile = (String, Vec<String>, Vec<String>);

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: cargo run -- <archivo.md>");
        std::process::exit(1);
    }

    let markdown_file = &args[1];

    let markdown = fs::read_to_string(markdown_file)?;

    let parser = Parser::new(&markdown);

    let mut current_path = vec!["./music".to_owned()];
    let mut current_heading_level = None;

    let mut links: Vec<MusicFile> = Vec::new();

    let re_list = Regex::new(
        r#"(?x)
        (https?://[^\s,]+)     # grupo 1: URL (hasta la primera coma o espacio)
        \s*,\s*                # separador coma con posibles espacios
        (.+)                   # grupo 2: todo lo demás (las etiquetas)
        "#,
    )?;

    for event in parser {
        match event {
            Event::Start(MdTag::Heading {
                level,
                id: _,
                classes: _,
                attrs: _,
            }) => {
                current_heading_level = Some(level);
            }
            Event::Text(text) => {
                if let Some(level) = current_heading_level.take() {
                    while current_path.len() > level as usize {
                        current_path.pop();
                    }
                    let safe_text = text.replace("/", "-");
                    current_path.push(safe_text);

                    let path = &current_path.join("/");
                    let dir = Path::new(path);
                    fs::create_dir_all(dir)?;
                } else if let Some(caps) = re_list.captures(&text) {
                    let url = caps.get(1).unwrap().as_str();
                    let rest = caps.get(2).unwrap().as_str();
                    let tags: Vec<String> = rest
                        .split(',')
                        .map(|s| s.trim().to_owned())
                        .filter(|s| !s.is_empty())
                        .collect();

                    links.push((url.to_owned(), current_path.clone(), tags));
                }
            }
            _ => {}
        }
    }
    let expanded_links = expand_urls(links);

    download_music(expanded_links);

    println!("\nFin!");

    Ok(())
}

fn expand_urls(links: Vec<MusicFile>) -> Vec<MusicFile> {
    links
        .par_iter()
        .flat_map(|(url, path, tags)| {
            println!("Analizando contenido de: {}", url);

            let output = Command::new("yt-dlp")
                .args(["--flat-playlist", "--print", "url", "--ignore-errors", url])
                .output();

            let mut new_links = Vec::new();

            match output {
                Ok(out) => {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    for line in stdout.lines() {
                        let single_url = line.trim();
                        if !single_url.is_empty() {
                            new_links.push((single_url.to_string(), path.clone(), tags.clone()));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error al expandir playlist {}: {}", url, e);
                }
            }

            new_links
        })
        .collect()
}

fn download_music(links: Vec<MusicFile>) {
    println!("Inciando descargas...");

    links.par_iter().for_each(|(url, current_path, tags)| {
        let path = current_path.join("/");
        let dir = Path::new(&path);
        println!("Descargando {url} en {:?}", path);
        let output = Command::new("yt-dlp")
            .args([
                "--no-overwrites",
                "-x",
                "--no-simulate",
                "--audio-format",
                "mp3",
                "-o",
                &(dir.join("%(title)s.%(ext)s").to_string_lossy()),
                "--print",
                "after_move:filepath",
                url,
            ])
            .output()
            .expect("Expected command completion");
        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            let filename = line.trim();
            if filename.is_empty() {
                continue;
            }

            let file_path = PathBuf::from(filename);

            if !file_path.exists() {
                eprintln!("Advertencia: El archivo no existe: {}", filename);
                continue;
            }

            let mut tag = match Tag::read_from_path(&file_path) {
                Ok(t) => t,
                Err(_) => Tag::new(),
            };

            if let Some(artist) = current_path.last() {
                tag.set_artist(artist);
            }
            tag.set_album(current_path.join(" - "));

            tag.add_frame(id3::frame::Comment {
                lang: "spa".to_owned(),
                description: "Tags".to_owned(),
                text: tags.join(", "),
            });

            if let Err(e) = tag.write_to_path(&file_path, Version::Id3v24) {
                eprintln!("Error escribiendo tags en {}: {}", filename, e);
            } else {
                println!("Tags aplicados a: {}", filename);
            }
        }
    });
}
