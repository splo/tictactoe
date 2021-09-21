extends Node

enum Mark {NONE, X, O}

onready var mark_placed1_player = $MarkPlaced1
onready var mark_placed2_player = $MarkPlaced2
onready var won_game_player = $WonGame
onready var draw_game_player = $DrawGame

func on_mark_placed(_coordinates: Vector2, mark: int):
	match mark:
		Mark.X:
			mark_placed1_player.play()
		Mark.O:
			mark_placed2_player.play()

func on_game_ended(winner: int, _winning_line: PoolVector2Array):
	match winner:
		Mark.NONE:
			draw_game_player.play()
		Mark.X:
			won_game_player.play()
		Mark.O:
			won_game_player.play()
