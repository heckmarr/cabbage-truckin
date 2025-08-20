extends Player


var scene = preload("res://spinning-godot-icon.tscn")
func _on_balete() -> void:
	self.queue_free()
func _enter_tree() -> void:
	var boss_timer = Timer.new()
	add_child(boss_timer)
	boss_timer.one_shot = true
	boss_timer.wait_time = 5
	boss_timer.connect("timeout", self._on_timer_timeout)
	boss_timer.start()	
func _on_timer_timeout() -> void:
	self.on_timer_done()
	var new_texture = preload("res://sprites/ghost-boss-angry.png")
	self.texture = new_texture
