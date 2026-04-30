# Sunshine

A single-file Rust TUI app that displays sundial mottoes with themed ASCII art.

## Build & Run

```
cargo build
cargo run
```

## Architecture

Everything lives in `src/main.rs`:

- **`Theme` enum** — `Sun`, `Shadow`, `Time`, `Mortality`, `Light`, `Wisdom`, `Work`, `Nature`
- **`Theme::raw_arts()`** — returns `&'static [&'static str]`, multiple art options per theme
- **`Theme::art(&self, rng, prev)`** — picks a random art piece and auto-pads all lines to equal width (required because ratatui's `Alignment::Center` centers each line independently). If `prev` is provided and the theme has multiple arts, avoids repeating the same padded output. The selection loop is bounded to `arts.len() * 2` attempts, falling back to accept a duplicate rather than hang
- **`TimeOfDay` enum** — `Dawn`, `Morning`, `Afternoon`, `Evening`, `Night`; computed from the current hour via `from_hour()`. Provides `accent()` (RGB color). All accent colors stay in the warm amber/yellow family except `Night` which desaturates to cool silver. `TimeOfDay::ALL` const array used for debug cycling.
- **`MOTTOS`** — `&[(Theme, &str, &str)]` array of `(theme, original, translation)` tuples
- **Shuffle bag** — mottoes are shuffled and walked through in order, reshuffled when exhausted (no repeats until all seen). On reshuffle, if the first element would repeat the last-shown motto, it is swapped to avoid back-to-back duplicates
- **History navigation** — `history: Vec<(usize, String)>` stores (MOTTOS index, art_text) pairs; `hist_pos` tracks current position. "Next" advances through history or draws from the shuffle bag; "Previous" walks back showing the exact same motto+art the user saw. History is capped at 200 entries to prevent unbounded memory growth; oldest entries are drained from the front when the cap is exceeded
- **Auto-advance** — if no key is pressed for 15 minutes, the app automatically advances to the next motto (same behavior as pressing `n`). Any keypress resets the timer. Uses `Instant` for monotonic elapsed-time tracking
- **Weather** — background thread fetches current weather from `wttr.in/?format=%C+%t` (auto-detects location via IP, no API key needed). Uses `ureq` with a 10-second request timeout via `AgentBuilder`. Result cached in `Arc<Mutex<Option<String>>>`, refreshed every 30 minutes. Displayed in top-left border as DarkGray text. Gracefully absent if fetch fails or hasn't completed yet
- **Layout** — border rendered on full area, content laid out within `outer.inner(area)` to preserve border pipes. Border and all accent text use the `TimeOfDay` accent color (shifts with time of day)
- **Tiered rendering** — checks `frame.area()` dimensions and degrades gracefully: full layout (art+motto+footer) >= `art_height+9`, compact (motto+footer, no art) >= 7, minimal (just translation) >= 3, "Terminal too small" below that or width < 20
- **Clock** — current time (`HH:MM`) displayed centered in the bottom border via `Block::title_bottom()`; event loop uses `event::poll(Duration::from_secs(1))` so the clock updates every second without keypresses
- **Bottom border layout** — clock (center, accent) | seen/total counter (right, DarkGray, hidden once all mottoes seen)
- **Terminal cleanup** — `main()` installs a panic hook and always runs `disable_raw_mode` + `LeaveAlternateScreen` after `run()` returns, regardless of success or error. A `signal_hook` SIGINT handler sets an `AtomicBool` quit flag so external signals also trigger clean exit. The `run()` function contains the event loop; `main()` owns setup and teardown.

## Key Patterns

- ASCII art is stored as `concat!()` blocks of `"line\n"` strings
- Art lines MUST be padded to equal width — the `art()` method handles this at runtime
- When adding new art: just add a `concat!(...)` entry to the relevant theme's slice in `raw_arts()`
- Artist attributions go in README.md acknowledgements section

## Controls

- `n` / `Space` / `→` — next motto
- `p` / `Backspace` / `←` — previous motto
- `q` / `Esc` / `Ctrl+C` — quit
- `d` — (hidden) cycle time-of-day debug override: Dawn → Morning → Afternoon → Evening → Night → real time
