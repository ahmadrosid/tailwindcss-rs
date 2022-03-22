# Lightweight Tailwindcss
I love tailwindcss it help me enjoying working on my blog [ahmadrosid.com](https://www.ahmadrosid.com). I create this project to learn more about tailwindcss and create lightweight version of it for fun.

```shell
tailwind-rs 0.1.0
Lightweight tailwindcss!

USAGE:
    tailwind-rs [OPTIONS] --input <INPUT> --output <OUTPUT>

OPTIONS:
    -h, --help               Print help information
    -i, --input <INPUT>      Source directories for html files!
    -o, --output <OUTPUT>    Css output path
    -V, --version            Print version information
    -w, --watch              Watch file changes
```

## Usage

Example cli usage:
```shell
tailwind-rs -i index.html -o output.css
```

## WIP
There's a lot of utility to be added. See [default-config.json](/src/default-config.json) for available utility.
