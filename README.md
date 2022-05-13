# slp-search

This program allows you to search through and filter your Slippi replay (.slp)
files quickly. Written in Rust using [peppi](https://github.com/hohav/peppi).

## Usage

`slp-search filter [OPTIONS] [REPLAYS]...`
`slp-search search [OPTIONS] [REPLAYS]...`

`slp-search -h` for more info

Specify one or more restrictions and supply one or more replay files. Ex:

`slp-search filter --pchar fox --pname Clown --ochar peach *.slp`
