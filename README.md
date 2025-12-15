# 🎵 Dnd Group Music

[![es](https://img.shields.io/badge/lang-es-yellow.svg)](README.es.md)

> Organize, download, and tag your music library automatically from a simple Markdown file.

This is a high-performance CLI tool written in **Rust**. It parses a `.md` file, creates a folder structure based on headers, downloads audio using `yt-dlp`, converts it to MP3, and applies ID3 metadata (Artist, Album, Custom Tags) automatically.

## 🚀 Quick Install

Go to the **[Releases](../../releases)** page and download the latest version.

### 🪟 For Windows (Bundle)
Download `MarkdownMusicDownloader_Win64.zip`.
1. Unzip the file.
2. Inside, you will find the executable along with `yt-dlp` and `ffmpeg` pre-configured.
3. Open a terminal in that folder and run:
```powershell
   .\music-downloader.exe my_list.md
```
No extra installation required.

### 🐧 For Linux

Download music-downloader-linux.tar.gz.
Extract the binary.
Install dependencies (ffmpeg and yt-dlp) using your package manager:
```Bash
sudo apt update && sudo apt install ffmpeg yt-dlp
```
Grant permissions and run:
```Bash
chmod +x music-downloader
./music-downloader my_list.md
```

### 📖 Markdown Format (.md)

The input file must follow this hierarchical structure:
```Markdown

# Main Folder (Genre)
## Subfolder (Style)
### Artist
[https://youtu.be/video1](https://youtu.be/video1) , tag1, tag2
[https://youtube.com/playlist](https://youtube.com/playlist)... , ost, soundtrack
```

### ✨ Features

- Map-Reduce for Playlists: Automatically expands YouTube playlists and downloads individual songs in parallel.

- No Runtime Dependencies: Written in Rust.

- Automatic ID3v2.4 Tagging:

    Artist: Extracted from the deepest folder name.

    Album: Full relative path.

    Comments: Custom tags defined in the Markdown file.

### 🛠️ Manual Build

If you prefer to build it from source:
```Bash

# Requires Rust installed
git clone [https://github.com/your-username/your-repo.git](https://github.com/your-username/your-repo.git)
cd your-repo
cargo build --release
```

📄 License

This project is licensed under the MIT License.
