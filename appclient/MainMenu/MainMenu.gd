extends Control

func _ready():
	print("and now Iḿ reading the player name => ", GameState.player_name)
	$VBoxContainer/Label.text = $VBoxContainer/Label.text.replace("{}", GameState.player_name)


func _on_Join_pressed():
	get_tree().change_scene("res://Lobby/Lobby.tscn")


func _on_New_pressed():
	get_tree().change_scene("res://Lobby/Lobby.tscn")
