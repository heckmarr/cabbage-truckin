extends Mobile
func bang_b():
	self.random_damage_emit()
func bang_a():
	self.damage_emit(50)
func _process(_delta):
	if Input.is_action_just_pressed("Pad-A"):
		print("A")
		self.bang_a()
	if Input.is_action_just_pressed("Pad-B"):
		print("B")
		self.bang_b()
	if Input.is_action_just_pressed("Pad-Y"):
		print("Y")
