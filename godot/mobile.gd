extends Mobile
func bang_a():
	self.damage_emit(50)
func _process(_delta):
	if Input.is_action_just_pressed("Pad-A"):
		self.bang_a()
	if Input.is_action_just_pressed("Pad-B"):
		print("B")
	if Input.is_action_just_pressed("Pad-X"):
		print("X")
	if Input.is_action_just_pressed("Pad-Y"):
		print("Y")
