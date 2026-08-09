# Game of Life

An CLI implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life) written in Rust.

## Build

```bash
cargo build --release
```

## Install

```bash
cargo install [--path .]
```

This installs the binary to `$CARGO_HOME` (defaults to `~/.cargo/bin`), so you can run it from anywhere.

## Usage

```bash
game-of-life -w,--width <width> -h,--height <height> -f,--file <filepath> [--fps <fps>]
```

Controls:

| Key   | Action |
| ----- | ------ |
| `Esc` | Quit   |

Press `Esc` to exit the simulation.

## License

This project is licensed under the terms of the MIT license, see [LICENSE](LICENSE) file for details.
