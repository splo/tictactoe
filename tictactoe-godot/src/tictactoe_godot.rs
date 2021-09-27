use std::vec::Vec;

use gdnative::{
    nativescript::property::{EnumHint, IntHint},
    prelude::*,
};
use tictactoe::{Action, Event, TicTacToe};

use crate::variant::{
    coordinates_to_variant, line_to_variant, mark_to_variant, no_mark_to_variant,
    vector_to_coordinates,
};

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct TicTacToeGodot {
    state: TicTacToe,
    #[property]
    last_clicked_coordinates: Vector2,
}

#[methods]
impl TicTacToeGodot {
    fn new(_owner: &Node) -> Self {
        TicTacToeGodot {
            state: TicTacToe::new(),
            last_clicked_coordinates: Vector2::default(),
        }
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "game_started",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "game_ended",
            args: &[
                SignalArgument {
                    name: "winner",
                    default: no_mark_to_variant(),
                    export_info: mark_enum_export_info(),
                    usage: PropertyUsage::DEFAULT,
                },
                SignalArgument {
                    name: "winning_line",
                    default: Variant::from_vector2_array(&Vector2Array::default()),
                    export_info: ExportInfo::new(VariantType::Vector2Array),
                    usage: PropertyUsage::DEFAULT,
                },
            ],
        });
        builder.add_signal(Signal {
            name: "mark_placed",
            args: &[
                SignalArgument {
                    name: "coordinates",
                    default: Variant::from_vector2(&Vector2::default()),
                    export_info: ExportInfo::new(VariantType::Vector2),
                    usage: PropertyUsage::DEFAULT,
                },
                SignalArgument {
                    name: "mark",
                    default: no_mark_to_variant(),
                    export_info: mark_enum_export_info(),
                    usage: PropertyUsage::DEFAULT,
                },
            ],
        });
    }

    #[export]
    fn _physics_process(&mut self, owner: &Node, delta: f64) {
        let actions = process_input(&self.last_clicked_coordinates);
        let events = self.state.update(delta, &actions);
        process_events(&events, owner);
    }
}

fn process_input(last_clicked_coordinates: &Vector2) -> Vec<Action> {
    let mut actions = Vec::<Action>::new();
    let input = Input::godot_singleton();
    if input.is_action_just_released("reset") {
        actions.push(Action::Reset);
    }
    if input.is_action_just_pressed("place_mark") {
        let coordinates = vector_to_coordinates(last_clicked_coordinates);
        if (0..3).contains(&coordinates.column) && (0..3).contains(&coordinates.row) {
            actions.push(Action::PlaceMark { coordinates })
        }
    }
    actions
}

fn process_events(events: &[Event], owner: &Node) {
    for event in events {
        match event {
            Event::GameStarted => {
                owner.emit_signal("game_started", &[]);
            }
            Event::GameEnded {
                winner,
                winning_line,
            } => {
                owner.emit_signal(
                    "game_ended",
                    &[
                        winner.map_or(no_mark_to_variant(), |mark| mark_to_variant(&mark)),
                        winning_line.map_or(
                            Variant::from_vector2_array(&Vector2Array::default()),
                            line_to_variant,
                        ),
                    ],
                );
            }
            Event::MarkPlaced { mark, coordinates } => {
                owner.emit_signal(
                    "mark_placed",
                    &[coordinates_to_variant(coordinates), mark_to_variant(mark)],
                );
            }
        }
    }
}

fn mark_enum_export_info() -> ExportInfo {
    IntHint::<i64>::export_info(IntHint::Enum(EnumHint::new(vec![
        "NONE".into(),
        "X".into(),
        "O".into(),
    ])))
}
