# scorelib

A safe, modular Rust library to parse and write Guitar Pro files.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
scorelib = { path = "../lib" }
```

Basic usage:

```rust
use scorelib::Song;
// Traits are re-exported in the root for convenience
use scorelib::SongTrackOps; 

fn main() {
    let data = std::fs::read("my_awesome_song.gp5").expect("Unable to read file");
    
    let mut song = Song::default();
    // Use the trait-based reader implementation
    song.read_gp5(&data).expect("Failed to parse file");
    
    println!("Song Title: {}", song.name);
    for track in &song.tracks {
        println!("Track: {}", track.name);
    }
}
```

## Features

- **GP3, GP4, GP5**: High-fidelity reading and writing support.
- **GP6/GP7 (.gp, .gpx)**: Reading support via GPIF import. **Note:** Writing is not yet supported for these formats.
- **MuseScore (.mscz)**: Basic XML/ZIP parsing.
- **Modular Design**: Separated into `model`, `io` (low-level primitives), and `audio` (MIDI).
- **Extensible**: Uses Rust traits to add format-specific functionality to the core `Song` model.

## Roadmap

- [x] Refactor into `model`, `io`, and `audio` modules.
- [x] Convert `impl Song` blocks into specialized traits.
- [x] Improve GP5 parsing (better handling of complex directions).
- [x] Support for GP6/7 (.gp/.gpx) formats (Read-only).
- [ ] Stabilize GP6/7 support (handle more complex effects).
- [ ] Full RSE support (Currently partial).
- [ ] Write support for newer formats (GPX/GP).
- [ ] Comprehensive documentation of the data model.
