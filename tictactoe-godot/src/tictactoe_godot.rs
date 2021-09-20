use gdnative::prelude::*;
use tictactoe::TicTacToe;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct TicTacToeGodot {
    state: TicTacToe,
}

#[methods]
impl TicTacToeGodot {
    fn new(_owner: &Node) -> Self {
        TicTacToeGodot {
            state: TicTacToe::new(),
        }
    }

    #[export]
    fn _physics_process(&mut self, _owner: &Node, _delta: f64) {}
}
