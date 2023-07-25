# AOCFetcher
A simple executable written in [Rust](https://www.rust-lang.org/en-US/tools/install) intended for fetching inputs of the [Advent of Code](https://adventofcode.com/) coding challenges.

## Getting Started

### Crates
* exitcode v1.1.2
* reqwest v0.11.18
* scraper v0.17.1
* toml v0.7.6

### Usage
Although intended for personal use, If for any reason you decide to use this, it should be simple
* Get the desired release or download the source code and build it
* Run the binary to generate a Fetch.toml file
* Get your session cookie from a browser and place it on the Fetch.toml
* Remember to configure the proper year on the config file
* Run and Enjoy. The contents will be downloaded to ./input by default

## Help

First and foremost: the code is sort of poorly written, but it gets the job done. Maybe I will work on it more over time, who knows...
With that in mind, feel free to modify and use it. And if you manage to use it and end up making it better/faster let me know, there can be no harm in learning from the mistakes of code written in a hurry, right?

## Version History

* 1.0
    * Initial Release
