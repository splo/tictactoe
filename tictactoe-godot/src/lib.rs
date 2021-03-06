mod tictactoe_godot;
mod variant;

use gdnative::prelude::{godot_init, InitHandle};
use tictactoe_godot::TicTacToeGodot;

fn init(handle: InitHandle) {
    handle.add_class::<TicTacToeGodot>();
}

godot_init!(init);
