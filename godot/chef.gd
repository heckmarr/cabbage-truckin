extends Mobiles


func _on_balete() -> void:
	self.queue_free()

func _enter_tree() -> void:
	if self.mob == "Chef":
		var new_texture = preload("res://sprites/chef-normal.png")
		self.texture = new_texture
		self.position = Vector2(100.0, 100.0)
func _on_sprite_2d_damage_all_mobiles(amount: int) -> void:
	self.mobile_damage_emit(amount)
