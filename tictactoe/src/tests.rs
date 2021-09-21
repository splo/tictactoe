use super::*;

#[test]
fn no_action_then_no_event() {
    let mut tictactoe = TicTacToe::default();
    assert_eq!(tictactoe.update(0.1, &(vec![])), vec![]);
}

#[test]
fn reset_then_game_started() {
    let mut tictactoe = TicTacToe::new();
    let actions = vec![Action::Reset];

    let events = tictactoe.update(0.1, &actions);

    assert_eq!(events, vec![Event::GameStarted]);
}

#[test]
fn place_single_mark_then_x_mark_placed() {
    let mut tictactoe = TicTacToe::new();
    let actions = vec![place_mark(0, 1)];

    let events = tictactoe.update(0.1, &actions);

    assert_eq!(
        events,
        vec![Event::MarkPlaced {
            coordinates: Coordinates { column: 0, row: 1 },
            mark: Mark::X
        }]
    );
}

#[test]
fn place_multiple_marks_then_all_mark_placed() {
    let mut tictactoe = TicTacToe::new();
    let actions = vec![place_mark(0, 0), place_mark(2, 2), place_mark(1, 0)];

    let events = tictactoe.update(0.1, &actions);

    assert_eq!(
        events,
        vec![
            Event::MarkPlaced {
                coordinates: Coordinates { column: 0, row: 0 },
                mark: Mark::X
            },
            Event::MarkPlaced {
                coordinates: Coordinates { column: 2, row: 2 },
                mark: Mark::O
            },
            Event::MarkPlaced {
                coordinates: Coordinates { column: 1, row: 0 },
                mark: Mark::X
            },
        ]
    );
}

#[test]
fn place_2_marks_at_same_coordinates_then_only_1_placed() {
    let mut tictactoe = TicTacToe::new();
    let actions = vec![place_mark(0, 0), place_mark(0, 0)];

    let events = tictactoe.update(0.1, &actions);

    assert_eq!(
        events,
        vec![Event::MarkPlaced {
            coordinates: Coordinates { column: 0, row: 0 },
            mark: Mark::X
        },]
    );
}

#[test]
fn place_marks_in_multiple_updates_then_all_marks_placed() {
    let mut tictactoe = TicTacToe::new();
    let actions1 = vec![place_mark(0, 0)];
    let actions2 = vec![place_mark(1, 1)];
    let actions3 = vec![place_mark(2, 0)];

    let events1 = tictactoe.update(0.1, &actions1);
    let events2 = tictactoe.update(0.1, &actions2);
    let events3 = tictactoe.update(0.1, &actions3);

    assert_eq!(
        events1,
        vec![Event::MarkPlaced {
            coordinates: Coordinates { column: 0, row: 0 },
            mark: Mark::X
        }]
    );
    assert_eq!(
        events2,
        vec![Event::MarkPlaced {
            coordinates: Coordinates { column: 1, row: 1 },
            mark: Mark::O
        }]
    );
    assert_eq!(
        events3,
        vec![Event::MarkPlaced {
            coordinates: Coordinates { column: 2, row: 0 },
            mark: Mark::X
        }]
    );
}

#[test]
fn x_winning() {
    let mut tictactoe = TicTacToe::new();
    let actions = vec![
        place_mark(0, 0),
        place_mark(1, 2),
        place_mark(1, 1),
        place_mark(2, 0),
        place_mark(2, 2),
    ];

    let events = tictactoe.update(0.1, &actions);

    assert!(
        events.contains(&Event::GameEnded {
            winner: Some(Mark::X),
            winning_line: Some((
                Coordinates { column: 0, row: 0 },
                Coordinates { column: 1, row: 1 },
                Coordinates { column: 2, row: 2 }
            )),
        }),
        "events: {:?}",
        events
    );
}

#[test]
fn o_winning() {
    let mut tictactoe = TicTacToe::new();
    let actions = vec![
        place_mark(1, 1),
        place_mark(0, 2),
        place_mark(1, 2),
        place_mark(0, 0),
        place_mark(2, 1),
        place_mark(0, 1),
    ];

    let events = tictactoe.update(0.1, &actions);

    assert!(
        events.contains(&Event::GameEnded {
            winner: Some(Mark::O),
            winning_line: Some((
                Coordinates { column: 0, row: 0 },
                Coordinates { column: 0, row: 1 },
                Coordinates { column: 0, row: 2 }
            )),
        }),
        "events: {:?}",
        events
    );
}

#[test]
fn o_winning2() {
    let mut tictactoe = TicTacToe::new();
    let actions = vec![
        place_mark(0, 0),
        place_mark(1, 0),
        place_mark(0, 1),
        place_mark(2, 0),
        place_mark(1, 1),
        place_mark(2, 1),
        place_mark(1, 2),
        place_mark(2, 2),
    ];

    let events = tictactoe.update(0.1, &actions);

    assert!(
        events.contains(&Event::GameEnded {
            winner: Some(Mark::O),
            winning_line: Some((
                Coordinates { column: 2, row: 0 },
                Coordinates { column: 2, row: 1 },
                Coordinates { column: 2, row: 2 }
            )),
        }),
        "events: {:?}",
        events
    );
}

#[test]
fn draw() {
    let mut tictactoe = TicTacToe::new();
    let actions = vec![
        place_mark(0, 0),
        place_mark(0, 1),
        place_mark(0, 2),
        place_mark(2, 0),
        place_mark(2, 1),
        place_mark(2, 2),
        place_mark(1, 0),
        place_mark(1, 1),
        place_mark(1, 2),
    ];

    let events = tictactoe.update(0.1, &actions);

    assert!(
        events.contains(&Event::GameEnded {
            winner: None,
            winning_line: None,
        }),
        "events: {:?}",
        events
    );
}

fn place_mark(column: usize, row: usize) -> Action {
    Action::PlaceMark {
        coordinates: Coordinates { column, row },
    }
}
