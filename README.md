# rosu-nps


A Rust library for analyzing note density and distribution in osu! beatmaps. Built on top of rosu-map, this library provides tools to calculate Notes Per Second (NPS) and create temporal distribution analysis of beatmap patterns.

## Features

- Calculate average Notes Per Second (NPS) for beatmaps
- Generate note density distributions with customizable block sizes
- Analyze note patterns using frequency-based sampling
- Fast performance (~12.6µs for 1000-block distribution analysis)
- Zero-copy operations on beatmap data

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rosu-nps = "0.1.0"
```

## Usage

```rust
use rosu_map::Beatmap;
use rosu_nps::{calculate_avg_nps, calculate_distribution, calculate_by_frequency};

fn main() {
    let beatmap = Beatmap::parse("path/to/map.osu").unwrap();
    
    // Calculate average NPS
    if let Some(nps) = calculate_avg_nps(&beatmap) {
        println!("Average NPS: {}", nps);
    }
    
    // Get distribution with 100 blocks
    if let Some(distribution) = calculate_distribution(&beatmap, 100) {
        println!("Note distribution: {:?}", distribution);
    }
    
    // Analyze with 10Hz sampling rate
    if let Some(frequency_dist) = calculate_by_frequency(&beatmap, 10.0) {
        println!("Frequency distribution: {:?}", frequency_dist);
    }
}
```

## Performance

Benchmarked on a sample beatmap with 1000 distribution blocks (0.1% precision):

```
time: [12.479 µs 12.660 µs 12.858 µs]
```


## License

MIT License

Copyright (c) 2024 Osef0760

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.