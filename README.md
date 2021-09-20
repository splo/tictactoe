# Tic Tac Toe

A simple [tic-tac-toe](https://en.wikipedia.org/wiki/Tic-tac-toe) two player game for Windows, macOS and Linux, with simple graphics and sounds.

## Development

### Organization

The code is split in 3 parts:

- `tictactoe`: a pure Rust library that hosts the core game logic.
- `tictactoe-gui`: the Godot project that defines all graphics, sounds and UI code, and generates the input actions.
- `tictactoe-godot`: a Rust library that provides glue code to link the Rust library and the Godot project together.
