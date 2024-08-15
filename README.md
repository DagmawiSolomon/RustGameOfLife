# Conway's Game of Life in Rust

[Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) is a cellular automaton devised by mathematician [John Conway](https://en.wikipedia.org/wiki/John_Horton_Conway). This implementation simulates the game using Rust, showcasing basic features of Rust programming and game simulation.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Running Tests](#running-tests)

## Overview

This project provides a Rust implementation of Conway's Game of Life. It features a basic simulation of the game on a 2D grid, where cells evolve according to specific rules. The grid is displayed in the terminal, and patterns can be initialized to observe different behaviors.

## Features

- Simulates Conway's Game of Life with standard rules.
- Displays the grid in the terminal with live updates.
- Clears and refreshes the terminal screen for each update.

## Installation

1. Ensure you have Rust installed. If not, follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).
2. Clone the repository:
   ```sh
   git clone https://github.com/DagmawiSolomon/RustGameOfLife.git
   ```
3. Navigate to the project directory:
   ```sh
   cd gameoflife
   ```

## Usage

1. Build the project:
   ```sh
   cargo build
   ```
2. Run the simulation:
   ```sh
   cargo run
   ```

## Running Tests

To ensure that everything is working correctly, you can run the tests provided with the project:

```sh
cargo test
```
