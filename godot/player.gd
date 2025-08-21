extends Player

var play = preload("res://sprites/ghost-boss-normal.png")

var scene = preload("res://spinning-godot-icon.tscn")
func _on_balete() -> void:
	self.queue_free()
