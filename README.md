<!-- omit in TOC -->
# tag-agents

> **Simulator for a multi agent system playing tag**

[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/reb/tag-agents/blob/main/LICENSE)


1. [About](#about)
2. [Usage](#usage)
3. [Examples](#examples)

## About

Simulate of the game of Tag in [Rust](https://www.rust-lang.org/), where every agent in the simulation can make their individual choices about their next action.

### Rules

The game of Tag is played normally between kids on a playground. To translate the normal rules to a Multi Agent System there have been some abstractions.

1. Movement happens on a 2D plane, the only actions allowed are `Stay`, `Up`, `Down`, `Left` and `Right`.
2. One agent is 'It', once that agent is within an [Euclidean distance](https://en.wikipedia.org/wiki/Euclidean_distance) of 1 away from another agent it will automatically tag that other agent.
3. An agent that has recently tagged another player is given immunity from being tagged until another tag has happened.

### Notes
* Agents may move on top of each other.
* Movement is not blocked in any way besides the boundaries of the play area. If an agent chooses to move into a boundary the agent will not move.

## Usage

To compile, first [install the Rust toolchain](https://www.rust-lang.org/tools/install). Then with `cargo run` it can be run in debug mode. Alternatively use `cargo build --release` to create a binary in the `target/release/` folder.

### Command line options

> When using `cargo run` add `--` before adding arguments (eg. `cargo run -- --help`)

| flags | description |
|---|---|
| `--help` | Print help information |
| `-a`, `--agents` | Number of agents to create [default: 10] |
| `-w`, `--width` | Width of the tag field [default: 10] |
| `-h`, `--height` | Height of the tag field [default: 10] |
| `-l`, `--length` | Length of the simulation [default: 100] |
| `-s`, `--speed` | Speed of the simulation, measured in steps per second. Uncapped if set to 0 [default: 8] |


## Examples


### Execution with defaults

```
> cargo run
████████████
█          █
█     OO   █
█          █
█       O  █
█          █
█        O █
█      OO  █
█  #       █
█   O   O  █
█       O  █
████████████
>
```

```
> tag-agents  -w 60 -h 20 -a 100
██████████████████████████████████████████████████████████████████████████████████
█    O OO      O  O           O          O               O    O    O     O    O  █
█   O           O                   O    O O    O          O       O       O     █
█ O       O  O                O     O     O             O   O        O           █
█     O         O      O                       O                     O           █
█ O      O             O             O  O O O           O            OO          █
█                                  O                    O         O              █
█           O                              OO                 O     O           O█
█                  O O    O              #    O      O   O        O       O  O   █
█           O              OO              O                   O                O█
█          O    O      O       O                         O  O   O                █
█  OO              O                        OO                     O        O    █
█            O    O O  O     O              O    O          O    O  O            █
█         O            O  O         O  O                       O               O █
█                      O  O                   O                O        O        █
█        O            O            O     O O  O     O        O   O     O         █
█                  O   O                                        O   O           O█
█                                   O    O      O OO                    O        █
█       O    O             O              O                 O                    █
█                                    O          O          O        O       O   O█
█                                                                         O   O  █
██████████████████████████████████████████████████████████████████████████████████
>
```