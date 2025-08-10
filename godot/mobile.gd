extends Mobile
func missile():
	self.hit_by_missile(50)
func _ready():
	self.take_damage(45)
	self.missile()
func _process(delta):
	if Input.is_action_just_pressed("Pad-A"):
		self.missile()
