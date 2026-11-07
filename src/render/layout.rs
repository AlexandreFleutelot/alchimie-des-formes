use bevy::math::Vec2;

pub fn current_offset(width:f32) -> Vec2 {
    Vec2::new(-width - 50., 50.)
}

pub fn target_offset() -> Vec2 {
    Vec2::new(50., 50.)
}