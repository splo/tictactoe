use gdnative::core_types::{Variant, Vector2, Vector2Array};
use tictactoe::{Coordinates, Line, Mark};

pub fn vector_to_coordinates(vector: &Vector2) -> Coordinates {
    Coordinates {
        column: vector.x as usize,
        row: vector.y as usize,
    }
}

fn coordinates_to_vector(coordinates: &Coordinates) -> Vector2 {
    Vector2::new(coordinates.column as f32, coordinates.row as f32)
}

pub fn mark_to_variant(mark: &Mark) -> Variant {
    Variant::from_i64(*mark as i64 + 1)
}

pub fn no_mark_to_variant() -> Variant {
    Variant::from_i64(0)
}

pub fn coordinates_to_variant(coordinates: &Coordinates) -> Variant {
    Variant::from_vector2(&coordinates_to_vector(coordinates))
}

pub fn line_to_variant(line: Line) -> Variant {
    let mut array = Vector2Array::new();
    array.push(coordinates_to_vector(&line.0));
    array.push(coordinates_to_vector(&line.1));
    array.push(coordinates_to_vector(&line.2));
    Variant::from_vector2_array(&array)
}
