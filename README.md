# AOCFetcher
A simple executable written in [Rust](https://www.rust-lang.org/en-US/tools/install) intended for fetching inputs of the [Advent of Code](https://adventofcode.com/) coding challenges.

### Crates
* exitcode v1.1.2
* reqwest v0.11.18
* scraper v0.17.1
* toml v0.7.6

### Usage
Although intended for personal use, If for any reason you decide to use this, it should be simple
* Download the source code and build it
* Run the binary to generate a Fetch.toml file
* [Get your session cookie](https://github.com/wimglenn/advent-of-code-wim/issues/1) and place it on the Fetch.toml
* Remember to set the proper year on the configuration file
* Run it, and the contents will be downloaded to `./input` directory by default
