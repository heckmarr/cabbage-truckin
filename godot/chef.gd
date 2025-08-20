extends Mobiles


func _on_balete() -> void:
	self.queue_free()
func _enter_tree() -> void:
	#self.position = Vector2(300.0, 200.0)
	var format_string = "Chef is at %s"
	var actual_string = format_string % self.position
	print(actual_string)

func _on_sprite_2d_damage_all_mobiles(amount: int) -> void:
	self.mobile_damage_emit(amount)
