extends Mobiles


func _on_balete() -> void:
	self.queue_free()

func _enter_tree() -> void:

	self.position = Vector2(650.0, 350.0)
func _on_sprite_2d_damage_all_mobiles(amount: int) -> void:
	self.mobile_damage_emit(amount)
