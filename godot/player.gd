extends Player

var play = preload("res://sprites/ghost-boss-normal.png")

func _on_balete() -> void:
	self.queue_free()
