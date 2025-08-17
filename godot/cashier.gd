extends Mobiles


func _on_balete() -> void:
	self.queue_free()

func _enter_tree() -> void:
	var new_texture = preload("res://sprites/cashier-normal.png")
	self.texture = new_texture
	self.position = Vector2(200.0, 400.0)
func _on_sprite_2d_damage_all_mobiles(amount: int) -> void:
	self.mobile_damage_emit(amount)
