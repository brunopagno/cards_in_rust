extends Node2D

func _ready():
	pass

#func _process(delta):
#	pass

func _on_GoBack_pressed():
	get_tree().change_scene("res://MainMenu/MainMenu.tscn")
