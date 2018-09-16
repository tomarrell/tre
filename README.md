# Tré
A simple, fast and interactive alternative to 'tree.' 

Tre doesn't aim for feature parity with tree necessarily, but aims to provide useful functionality on top of standard tree to speed up command-line file and directory navigation.

## Features 
- Ignores hidden directories by default
- Parses .gitignore and ignores matching files/dirs
- Colorized output, easier to distinguish node types
- 25% shorted command than `tree`!

# Installation
Currently installation is only from source.

```bash
$ git clone [this repo]

$ cd tre

$ cargo install

$ tre [options]
```

# Benchmarks
Benchmarks run using Hyperfine `1.2.0` with 4 warmup runs before any recorded runs to warm the cache.

```
Benchmark #1: tree ~ -L 4

  Time (mean ± σ):     276.7 ms ±   7.4 ms    [User: 172.6 ms, System: 100.8 ms]

  Range (min … max):   265.5 ms … 287.9 ms
```

```
Benchmark #1: tre ~ -l 4

  Time (mean ± σ):     352.1 ms ±   6.7 ms    [User: 197.8 ms, System: 150.9 ms]

  Range (min … max):   344.1 ms … 365.7 ms
```

# License
Licensed under **MIT** *or* **GNU GPL v3.0**, at your discretion.
