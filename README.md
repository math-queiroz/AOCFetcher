# 🎄AOCFetcher🦀
A simple executable written in [Rust](https://www.rust-lang.org/en-US/tools/install) intended for fetching inputs of the [Advent of Code](https://adventofcode.com/) coding challenges.

## Features
- Fetch all `inputs/prompts` sequentially by running the executable with no parameters
- Fetch specific days by passing a number as argument, for example `./fetch 01` or `./fetch 1`
- Skips already present `inputs/prompts`
- Updates prompt if it changes since the last time ran
- Configurable input file format extension

## Crates
* exitcode v1.1.2
* reqwest v0.11.18
* scraper v0.17.1
* toml v0.7.6

## Usage
Although intended for personal use, If for any reason you decide to use this, it should be simple
* Download the source code and build it with `cargo build --release`
* Get the binary from traget/release/fetch and run it to generate a Fetch.toml file
* [Get your session cookie](https://github.com/wimglenn/advent-of-code-wim/issues/1) and define it on the Fetch.toml
  * Also remember to set the proper year on the configuration file
* Run it, and the contents will be downloaded to `./input` directory by default
