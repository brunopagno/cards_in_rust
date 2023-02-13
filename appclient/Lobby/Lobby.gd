extends Control

func _on_Leave_pressed():
	get_tree().change_scene("res://MainMenu/MainMenu.tscn")


func _on_ReadyCheck_pressed():
	get_tree().change_scene("res://InGame/InGame.tscn")
