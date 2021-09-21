extends GridContainer

var winning_line: PoolVector2Array = [] setget set_winning_line

func _draw():
	if winning_line.size() == 3:
		var panel1 = get_child(int(winning_line[0].x + winning_line[0].y * 3))
		var panel3 = get_child(int(winning_line[2].x + winning_line[2].y * 3))
		var panel1_size: Vector2 = panel1.get_rect().size
		var coordinates1: Vector2 = panel1.get_rect().position + (panel1_size / 2)
		var coordinates3: Vector2 = panel3.get_rect().position + (panel3.get_rect().size / 2)
		var direction: Vector2 = coordinates3 - coordinates1
		coordinates1 = coordinates1 - direction / 8
		coordinates3 = coordinates3 + direction / 8
		var width: float = min(panel1_size.x, panel1_size.y) / 8
		draw_line(coordinates1, coordinates3, Color("#80f47e1b"), width, true)

func set_winning_line(value):
	winning_line = value
	self.update()
