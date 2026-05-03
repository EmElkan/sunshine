# Sunshine

A TUI that displays sundial mottoes paired with themed ASCII art.

Built with [Ratatui](https://ratatui.rs) and [Crossterm](https://github.com/crossterm-rs/crossterm).


https://github.com/user-attachments/assets/18399dce-c21d-44c2-9336-50cddb4fa903


## Usage

```
cargo run
```

### Controls

| Key | Action |
|-----|--------|
| `n` / `Space` / `→` | Next motto |
| `p` / `Backspace` / `←` | Previous motto |
| `q` / `Esc` / `Ctrl+C` | Quit |

### Features

- Current time displayed in the bottom border, updates every second
- Live weather display in the top border via wttr.in, refreshed every 30 minutes
- Motto history, navigate back to previously seen mottoes with the exact same art
- Auto-advance to the next motto after 15 minutes idle
- Seen/total motto counter in the bottom border (hides once all mottoes seen)
- Graceful small-terminal handling, degrades from full art to compact to minimal as the terminal shrinks

## Acknowledgements

ASCII art sourced from [asciiart.eu](https://www.asciiart.eu/) and [asciiart.website](https://asciiart.website/), with works by the following artists:

- Hayley Jane Wakenshaw
- Joan Stark
- Shanaka Dias
- Dustin Slater
- jim
- Jiri Matejicek
- Hamilton Furtado

Additional art from community contributions on asciiart.eu and asciiart.website. All ASCII art is used for non-commercial, decorative purposes.

The collection draws from classical sundial inscriptions, many sourced from [Alfred H. Hyatt's *"Though Silent, I Speak: A Book of Sundial Mottoes"* (1903)](https://publicdomainreview.org/collection/sundial-mottoes/).

Built with [Claude Code](https://claude.ai/code).
