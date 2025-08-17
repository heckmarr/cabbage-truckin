extends Mobiles


func _on_balete() -> void:
	self.queue_free()
func _enter_tree() -> void:
	if self.mob == "Cashier":
		var new_texture = preload("res://sprites/cashier-normal.png")
		self.texture = new_texture
		self.position = Vector2(250.0, 300.0)
func _on_sprite_2d_damage_all_mobiles(amount: int) -> void:
	self.mobile_damage_emit(amount)
