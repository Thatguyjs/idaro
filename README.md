# idaro
An easy-to-use static site generator for Markdown pages

## Usage
| Command | Args | Defaults | Description |
|---------|------|----------|-------------|
|   run   | path? <br> addr? | "./src" <br> "127.0.0.1:8080" | Hosts a server at `addr`, and live-compiles any Markdown files to HTML |
|  build  | source-path? <br> build-path? | "./src" <br> "./build" | Builds the site, converting all Markdown files into HTML and copying remaining files into `build-path` |
