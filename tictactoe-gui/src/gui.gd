extends ColorRect

enum Mark {NONE, X, O}

onready var grid_node = $MainMarginContainer/MainVBoxContainer/BoardContainer/BoardMarginContainer/BoardHBoxContainer/GridContainer
onready var x_turn_node = $MainMarginContainer/MainVBoxContainer/ScoreContainer/ScoreMarginContainer/ScoreCenterContainer/ScoreHBoxContainer/XPanelContainer
onready var x_score_node = $MainMarginContainer/MainVBoxContainer/ScoreContainer/ScoreMarginContainer/ScoreCenterContainer/ScoreHBoxContainer/XPanelContainer/XHBoxContainer/XScoreLabel
onready var o_turn_node = $MainMarginContainer/MainVBoxContainer/ScoreContainer/ScoreMarginContainer/ScoreCenterContainer/ScoreHBoxContainer/OPanelContainer
onready var o_score_node = $MainMarginContainer/MainVBoxContainer/ScoreContainer/ScoreMarginContainer/ScoreCenterContainer/ScoreHBoxContainer/OPanelContainer/OHBoxContainer/OScoreLabel
onready var game_logic_node = get_parent()

var current_turn = Mark.NONE setget set_current_turn
var game_over: bool = false

func _ready():
	on_game_started()

func _input(event: InputEvent):
	if event is InputEventMouseButton:
		if game_over and event.pressed:
			Input.action_press("reset")
			Input.action_release("reset")
		elif event.button_index == BUTTON_LEFT:
			if event.pressed:
				var clicked_coordinates = global_position_to_mark_coordinates(event.position)
				if clicked_coordinates != null:
					game_logic_node.last_clicked_coordinates = clicked_coordinates
					Input.action_press("place_mark")
			elif Input.is_action_pressed("place_mark"):
				Input.action_release("place_mark")

func set_current_turn(mark):
	current_turn = mark
	focus_current_turn()

func global_position_to_mark_coordinates(position: Vector2):
	var i: int = 0
	for panel in grid_node.get_children():
		if panel.get_global_rect().has_point(position):
			#warning-ignore:integer_division
			return Vector2(i % 3, i / 3)
		i += 1
	return null

func focus_current_turn():
	x_turn_node.get_stylebox("panel").set_border_color(Color("#f47e1b") if current_turn == Mark.X else Color("#00000000"))
	o_turn_node.get_stylebox("panel").set_border_color(Color("#f47e1b") if current_turn == Mark.O else Color("#00000000"))

func on_game_started():
	self.current_turn = Mark.X
	self.game_over = false
	grid_node.winning_line = []
	for child in grid_node.get_children():
		child.set_mark(Mark.NONE)
	
func on_game_ended(winner: int, winning_line: PoolVector2Array):
	if winner == Mark.X:
		self.current_turn = Mark.X
		grid_node.winning_line = winning_line
		x_score_node.set_text(str(int(x_score_node.get_text()) + 1))
	elif winner == Mark.O:
		self.current_turn = Mark.O
		grid_node.winning_line = winning_line
		o_score_node.set_text(str(int(o_score_node.get_text()) + 1))
	else:
		self.current_turn = Mark.NONE
	self.game_over = true

func on_mark_placed(coordinates: Vector2, mark: int):
	var panel = grid_node.get_child(coordinates.x + coordinates.y * 3)
	panel.set_mark(mark)
	self.current_turn = Mark.X if mark == Mark.O else Mark.O
