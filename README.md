# aoc-kit

Helper library to work through [Advent of Code](https://adventofcode.com) puzzles

## Available Commands

### `aoc-kit init`

Initialize your advent of code solution directory. _Must be run before other commands_.

- `-t` `--token` _(required)_ your advent of code session token
- `-y` `--year` _(optional)_ the year of puzzles you want to work through

### `aoc-kit scaffold`

- `-d` `--day` _(optional)_ the puzzle day to scaffold
- `-o` `--open` _(optional)_ if true, opens [https://adventofcode.com](https://adventofcode.com) to the scaffolded puzzle

### `aoc-kit submit`

- `-d` `--day` _(optional)_ the puzzle day to submit
- `-p` `--part` _(required)_ the puzzle part (a, b) to submit
- `-o` `--open` _(optional)_ if true, opens [https://adventofcode.com](https://adventofcode.com) to the next puzzle
