extends Control

func _ready():
	$CenterContainer/HBoxContainer/LineEdit.grab_focus()

func _on_Confirm_pressed():
	var name_input = get_node("CenterContainer/HBoxContainer/LineEdit")
	if name_input.text:
		GameState.player_name = name_input.text
		print("the game state was set to => ", GameState.player_name)
		get_tree().change_scene("res://MainMenu/MainMenu.tscn")
	else:
		$CenterContainer/HBoxContainer/LineEdit/ErrorLabel.visible = true
