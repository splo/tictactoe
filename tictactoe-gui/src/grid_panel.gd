tool
extends Control

enum Mark {NONE, X, O}

export(Mark) var mark = Mark.NONE setget set_mark
export var x_color: Color = Color("#3978a8")
export var o_color: Color = Color("#f4cca1")

func _draw():
	match self.mark:
		Mark.X:
			draw_x_at_center()
		Mark.O:
			draw_o_at_center()

func set_mark(value):
	mark = value
	self.update()

func draw_o_at_center():
	var size: Vector2 = self.get_rect().size
	var min_size: float = min(size.x, size.y)
	var width: float = min_size / 8
	draw_arc(size / 2, min_size / 2 - 2 * width, 0, 2 * PI + 0.1, 31, o_color, width, true)

func draw_x_at_center():
	var size: Vector2 = self.get_rect().size
	var min_size: float = min(size.x, size.y)
	var width: float = min_size / 8
	draw_cross(size / 2, min_size, 0.0, x_color, width)

func draw_cross(position: Vector2, size: float, shift: float, color: Color, width: float):
	var offset = int(size / 2 - width * 2 + shift)
	var top_left = Vector2(position.x - offset, position.y - offset)
	var bottom_right = Vector2(position.x + offset, position.y + offset)
	var bottom_left = Vector2(position.x - offset, position.y + offset)
	var top_right = Vector2(position.x + offset, position.y - offset)
	draw_line(top_left, bottom_right, color, width + shift * 2, true)
	draw_line(bottom_left, top_right, color, width + shift * 2, true)
