# Spotty

A modern Terminal User Interface (TUI) for Spotify, built with Rust.

## Features

- **Multi-view Navigation** — Home, Search, Queue, and Lyrics views
- **Playback Control** — Play, pause, next, previous, volume, shuffle, repeat
- **Playlist & Library** — Browse playlists, liked songs, saved albums, artists, and podcasts
- **Device Management** — Switch playback between multiple Spotify-connected devices
- **Search** — Search across tracks, artists, albums, and playlists
- **Lyrics** — Display song lyrics with scroll support
- **Context Actions** — Quick-action menus for tracks, albums, artists, playlists, and episodes

## Requirements

- Rust (edition 2024)
- A Spotify Premium account
- Spotify API credentials (Client ID & Secret)

## Installation

```bash
git clone https://github.com/vquclinh/spotty
cd spotty
cargo build --release
```

## Configuration

Copy `.env.example` to `.env` and fill in your Spotify credentials:

```env
CLIENT_ID=your_client_id
CLIENT_SECRET=your_client_secret
```

Then run:

```bash
cargo run --release
```

## Keybindings

### Global

| Key | Action |
|-----|--------|
| `?` | Toggle help popup |
| `q` / `Ctrl+C` | Quit |
| `g` | Open quick actions popup |

### View Switching

| Key | Action |
|-----|--------|
| `H` | Home view |
| `S` | Search view |
| `Q` | Queue view |
| `L` | Lyrics view |

### Panel Focus

| Key | Action |
|-----|--------|
| `Tab` | Cycle focus between panels |
| `1` | Focus sidebar / input |
| `2` | Focus playlists / results |
| `3` | Focus main content |
| `4` | Focus playbar |

### Playback

| Key | Action |
|-----|--------|
| `Space` | Play / Pause |
| `n` | Next track |
| `p` | Previous track |
| `+` | Volume up |
| `-` | Volume down |
| `r` | Cycle repeat mode (Off → Context → Track) |
| `s` | Toggle shuffle |

### List Navigation

| Key | Action |
|-----|--------|
| `j` / `Down` | Move down |
| `k` / `Up` | Move up |
| `Ctrl+d` | Jump down 10 items |
| `Ctrl+u` | Jump up 10 items |
| `h` / `Left` | Move left / previous tab |
| `l` / `Right` | Move right / next tab |
| `Enter` | Select item |
| `Esc` | Close popup / go back |
| `b` / `Backspace` | Go back to playlist menu |

### Actions

| Key | Action |
|-----|--------|
| `t` | Open action menu for selected item |
| `Enter` | Confirm action / play selected |

#### Action menu options depend on item type:

| Item | Available Actions |
|------|-------------------|
| Track / Episode | Play Now, Add to Queue, Add to Playlist, Save/Remove from Library, Go to Album |
| Album | Play Now, Add to Queue, Save/Remove from Library |
| Artist | Follow / Unfollow |
| Playlist | Play Now, Add to Queue |

### Search View

| Key | Action |
|-----|--------|
| `Enter` | Execute search |
| `Tab` | Cycle result categories (Tracks → Artists → Albums → Playlists) |
| `Esc` | Return to search input |

### Device Selection

| Key | Action |
|-----|--------|
| `j` / `Down` | Next device |
| `k` / `Up` | Previous device |
| `Enter` | Transfer playback to selected device |

### Playlist Selector (Add to Playlist)

| Key | Action |
|-----|--------|
| `j` / `Down` | Move down |
| `k` / `Up` | Move up |
| `Enter` | Add item to selected playlist |
| `Esc` / `t` | Close |

## Tech Stack

| Component | Library |
|-----------|---------|
| TUI rendering | [ratatui](https://github.com/ratatui-org/ratatui) |
| Terminal events | [crossterm](https://github.com/crossterm-rs/crossterm) |
| Spotify Web API | [rspotify](https://github.com/ramsayleung/rspotify) |
| Audio playback | [librespot](https://github.com/librespot-org/librespot) |
| Async runtime | [tokio](https://tokio.rs) |

## Project Structure

```
src/
├── app/          # Application state and routes
├── audio/        # Audio playback and Spotify auth
├── event/        # Event system
├── handlers/     # Keyboard input handlers per view
├── network/      # Spotify API client
├── ui/           # Rendering logic, widgets, and popups
└── lib.rs        # App initialization and main event loop
```

## Authors

- Vo Quoc Linh
- night0

## License

MIT
