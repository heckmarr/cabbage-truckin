extends Player


func _on_balete() -> void:
	self.queue_free()
func _enter_tree() -> void:
	var game_start = Timer.new()
	add_child(game_start)
	game_start.one_shot = true
	game_start.wait_time = 5
	game_start.connect("timeout", self._on_timer_timeout)
	game_start.start()	
func _on_timer_timeout() -> void:
	self.on_timer_done()
	var new_texture = preload("res://sprites/ghost-boss-angry.png")
	self.texture = new_texture
