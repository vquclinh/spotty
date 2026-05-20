# Spotty

Spotty is a modern terminal user interface for Spotify, built with Rust. It brings playback control, library browsing, search, queue management, device transfer, and lyrics into a keyboard-driven TUI.

Spotty is an unofficial Spotify client and is not affiliated with Spotify AB.

## Features

- Keyboard-first Spotify playback: play, pause, skip, volume, shuffle, and repeat.
- Library access for playlists, liked songs, saved albums, followed artists, and saved podcasts.
- Home view with top tracks, top artists, and recently played tracks.
- Search across tracks, artists, albums, and playlists.
- Queue view with live playback context.
- Device selector for transferring playback between Spotify Connect devices.
- Synced and unsynced lyrics support through Spotify metadata.

## Requirements

- Rust 1.85+ with edition 2024 support.
- A Spotify Premium account for playback control and streaming.
- A Spotify Developer application with a redirect URI configured.
- Access to a browser for OAuth authorization.

## Installation

Spotty is intended to run on Linux, macOS, and Windows. Linux is the primary development environment; macOS and Windows support may depend on terminal, browser, and audio backend behavior.

Install Rust from [rustup.rs](https://rustup.rs), then create a Spotify application in the [Spotify Developer Dashboard](https://developer.spotify.com/dashboard). Add a redirect URI such as:

```text
http://localhost:8888/callback
```

Create `.env` from `.env.example`, then fill in your Spotify application values:

```env
RSPOTIFY_CLIENT_ID=your_spotify_client_id
RSPOTIFY_REDIRECT_URI=http://localhost:8888/callback

# Optional for PKCE, but supported if your Spotify app uses one.
RSPOTIFY_CLIENT_SECRET=your_spotify_client_secret
```

On first launch, Spotty opens a Spotify authorization page in your browser. OAuth tokens are cached in `.spotify_token_cache.json`, and audio/session data is cached in `.spotty_cache/`.

### Linux

```bash
git clone https://github.com/vquclinh/spotty.git
cd spotty
cp .env.example .env
cargo run --release
```

Build a release binary:

```bash
cargo build --release
./target/release/spotty
```

### macOS

```bash
git clone https://github.com/vquclinh/spotty.git
cd spotty
cp .env.example .env
cargo run --release
```

Build a release binary:

```bash
cargo build --release
./target/release/spotty
```

### Windows

Use PowerShell:

```powershell
git clone https://github.com/vquclinh/spotty.git
cd spotty
Copy-Item .env.example .env
cargo run --release
```

Build a release binary:

```powershell
cargo build --release
.\target\release\spotty.exe
```

## Keyboard Shortcuts

Use `?` inside Spotty to open the in-app help popup.

### Essential

| Key | Action |
| --- | --- |
| `q` | Quit |
| `?` | Toggle help |
| `g` | Open quick actions |
| `Esc` | Close popup |
| `Enter` | Select or confirm |
| `Tab` | Cycle focus between panels |
| `1` / `2` / `3` / `4` | Focus panel 1 / 2 / 3 / 4 |

### Views

| Key | Action |
| --- | --- |
| `H` | Home |
| `S` | Search |
| `Q` | Queue |
| `L` | Lyrics |

### Playback

| Key | Action |
| --- | --- |
| `Space` | Play or pause |
| `n` | Next track |
| `p` | Previous track |
| `+` / `-` | Increase or decrease volume |
| `s` | Toggle shuffle |
| `r` | Cycle repeat mode |

### Navigation

| Key | Action |
| --- | --- |
| `j` / `Down` | Move down |
| `k` / `Up` | Move up |
| `h` / `Left` | Move left or switch to the previous tab |
| `l` / `Right` | Move right or switch to the next tab |
| `Ctrl+d` | Jump down 10 items |
| `Ctrl+u` | Jump up 10 items |

### Search

| Key | Action |
| --- | --- |
| `Enter` | Run search |
| `Tab` | Cycle result groups: tracks, artists, albums, playlists |
| `Esc` | Return to search input |

### Device Selector

Open quick actions with `g`, then press `t` to transfer playback.

| Key | Action |
| --- | --- |
| `j` / `Down` | Next device |
| `k` / `Up` | Previous device |
| `Enter` | Transfer playback |

## Development

```bash
cargo check
cargo fmt
cargo clippy
```

Run the application locally with:

```bash
cargo run
```

Build an optimized binary with:

```bash
cargo build --release
```

The release binary is written to `target/release/spotty`.

## Tech Stack

| Area | Library |
| --- | --- |
| TUI rendering | [ratatui](https://github.com/ratatui-org/ratatui) |
| Terminal events | [crossterm](https://github.com/crossterm-rs/crossterm) |
| Spotify Web API | [rspotify](https://github.com/ramsayleung/rspotify) |
| Audio playback | [librespot](https://github.com/librespot-org/librespot) |
| Async runtime | [tokio](https://tokio.rs) |

## Project Structure

```text
src/
  app/        Application state, routes, and view models
  audio/      Librespot authentication, player, and audio events
  event/      Event primitives
  handlers/   Keyboard handlers by view
  network/    Spotify API client, requests, and response models
  ui/         Layouts, widgets, popups, and rendering
  lib.rs      Application bootstrap and main event loop
  main.rs     Binary entrypoint
```

## Authors

- Vo Quoc Linh (vquclinh)
- Nguyen Thanh Nhat (night0)

## License

MIT
