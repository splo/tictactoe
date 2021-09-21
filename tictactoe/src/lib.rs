use std::vec::Vec;

#[cfg(test)]
mod tests;

/// A mark that can be placed by a player
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Mark {
    X,
    O,
}

/// The location where marks can be placed
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Coordinates {
    pub column: usize,
    pub row: usize,
}

/// A line of mark coordinates
pub type Line = (Coordinates, Coordinates, Coordinates);

/// The actions that can be applied to a tic-tac-toe game
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Action {
    /// Start a new tic-tac-toe game
    Reset,
    /// Place a mark at specific coordinates
    PlaceMark { coordinates: Coordinates },
}

/// The events that can happen during a tic-tac-toe game
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Event {
    /// A tic-tac-toe game just started
    GameStarted,
    /// A tic-tac-toe game just finished, maybe with a winner
    GameEnded {
        winner: Option<Mark>,
        winning_line: Option<Line>,
    },
    /// A mark has been placed
    MarkPlaced {
        mark: Mark,
        coordinates: Coordinates,
    },
}

/// A tic-tac-toe game
pub struct TicTacToe {
    grid: [Option<Mark>; 9],
    turn: Mark,
    running: bool,
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            grid: Default::default(),
            turn: Mark::X,
            running: true,
        }
    }

    /// Updates the simulation and returns the events that happened
    ///
    /// # Arguments
    ///
    /// * `delta` - How long to advance the simulation step, in seconds
    /// * `actions` - The actions to apply to the simulation
    pub fn update(&mut self, _delta: f64, actions: &[Action]) -> Vec<Event> {
        let mut events = Vec::<Event>::new();
        for action in actions {
            match action {
                Action::Reset => {
                    self.grid = Default::default();
                    self.turn = Mark::X;
                    self.running = true;
                    events.push(Event::GameStarted);
                }
                Action::PlaceMark { coordinates } => {
                    let index: usize = to_index(coordinates);
                    if self.running && index < 9 && self.grid[index].is_none() {
                        self.grid[index] = Some(self.turn);
                        events.push(Event::MarkPlaced {
                            coordinates: *coordinates,
                            mark: self.turn,
                        });
                        let line = self.find_wining_line();
                        if line.is_some() {
                            self.running = false;
                            events.push(Event::GameEnded {
                                winner: Some(self.turn),
                                winning_line: line,
                            });
                        } else if self.grid_is_full() {
                            self.running = false;
                            events.push(Event::GameEnded {
                                winner: None,
                                winning_line: None,
                            });
                        } else {
                            self.turn = if self.turn == Mark::X {
                                Mark::O
                            } else {
                                Mark::X
                            };
                        }
                    }
                }
            }
        }
        events
    }

    fn find_wining_line(&self) -> Option<Line> {
        all_lines()
            .into_iter()
            .find(|line| self.line_is_winning(line))
    }

    fn line_is_winning(&self, line: &Line) -> bool {
        let turn_mark = Some(self.turn);
        self.grid[to_index(&line.0)] == turn_mark
            && self.grid[to_index(&line.1)] == turn_mark
            && self.grid[to_index(&line.2)] == turn_mark
    }

    fn grid_is_full(&self) -> bool {
        self.grid.iter().filter_map(|&o| o).count() == 9
    }
}

fn all_lines() -> Vec<(Coordinates, Coordinates, Coordinates)> {
    let mut lines: Vec<Line> = Vec::new();
    for index in 0..3 {
        // Horizontal checks.
        lines.push(build_line(0, index, 1, index, 2, index));
        // Vertical checks.
        lines.push(build_line(index, 0, index, 1, index, 2));
    }
    lines.push(build_line(0, 0, 1, 1, 2, 2));
    lines.push(build_line(2, 0, 1, 1, 0, 2));
    lines
}

impl Default for TicTacToe {
    fn default() -> Self {
        Self::new()
    }
}

fn build_line(
    col0: usize,
    row0: usize,
    col1: usize,
    row1: usize,
    col2: usize,
    row2: usize,
) -> Line {
    (
        Coordinates {
            column: col0,
            row: row0,
        },
        Coordinates {
            column: col1,
            row: row1,
        },
        Coordinates {
            column: col2,
            row: row2,
        },
    )
}

fn to_index(coordinates: &Coordinates) -> usize {
    coordinates.column + coordinates.row * 3
}
