extends Player
func _process(delta):
	if Input.is_action_just_pressed("Pad-X"):
		print("X")
		self.free()
