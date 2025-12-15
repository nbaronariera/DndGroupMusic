# 🎵 Dnd Group Music
[![en](https://img.shields.io/badge/lang-en-yellow.svg)](README.md)

> Organiza, descarga y etiqueta tu biblioteca musical automáticamente a partir de un simple archivo Markdown.

Esta herramienta CLI escrita en **Rust** lee un archivo `.md`, crea la estructura de carpetas automáticamente, descarga el audio con `yt-dlp`, convierte a MP3 y aplica metadatos ID3 (Artista, Álbum, Tags) en paralelo.

## 🚀 Instalación Rápida

Ve a la sección de **[Releases](../../releases)** y descarga la última versión.

### 🪟 Para Windows (Todo incluido)
Descarga el archivo `MarkdownMusicDownloader_Win64.zip`.
1. Descomprime el ZIP.
2. Dentro encontrarás el ejecutable junto con `yt-dlp` y `ffmpeg` ya configurados.
3. Abre una terminal en esa carpeta y ejecuta:
```powershell
   .\music-downloader.exe mi_lista.md
```
No necesitas instalar nada extra.

### 🐧 Para Linux

Descarga el archivo music-downloader-linux.tar.gz.
Extrae el binario.
Instala las dependencias (ffmpeg y yt-dlp) usando tu gestor de paquetes:
```Bash
  sudo apt update && sudo apt install ffmpeg yt-dlp
```
Dale permisos y ejecuta:
```Bash
  chmod +x music-downloader
  ./music-downloader mi_lista.md
```

## 📖 Formato del Markdown (.md)

El archivo debe seguir esta estructura jerárquica:
```Markdown
# Carpeta Principal (Género)
## Subcarpeta (Estilo)
### Artista
[https://youtu.be/video1](https://youtu.be/video1) , tag1, tag2
[https://youtube.com/playlist](https://youtube.com/playlist)... , ost, soundtrack
```

## ✨ Características Técnicas

- Map-Reduce para Playlists: Expande playlists de YouTube y descarga canciones individuales en paralelo.

- Sin Dependencias de Runtime: Escrito en Rust.

- Etiquetado ID3v2.4 Automático:

    Artista: Nombre de la carpeta contenedora.

    Álbum: Ruta relativa completa.

    Comentarios: Tags personalizados definidos en el Markdown.

### 🛠️ Compilación Manual

Si prefieres compilarlo tú mismo desde el código fuente:
```Bash
# Requiere Rust instalado
git clone [https://github.com/tu-usuario/tu-repo.git](https://github.com/tu-usuario/tu-repo.git)
cd tu-repo
cargo build --release
```

## 📄 Licencia
Este proyecto está bajo la licencia MIT.
