use crate::stuff;

extern crate minifb;
use minifb::{Key, Window, WindowOptions};

pub fn movement(window: &Window, angle: f32, position: stuff::Vector, map: &[&str; 16]) -> (f32, stuff::Vector) {
    let f32 movement_speed = 1;
    let f32 rotation_speed = 0.05;

    let mut dx: f32 = 0;
    let mut dy: f32 = 0;
    if window.is_key_down(Key::W) {dy+=movement_speed}
    if window.is_key_down(Key::S) {dy-=movement_speed}
    if window.is_key_down(Key::D) {dx+=movement_speed}
    if window.is_key_down(Key::A) {dx-=movement_speed}

    let mut delta_position = stuff::Vector::from_angle(angle).scalar_mul(dy) + stuff::Vector::from_angle(angle + (3.14159 / 2)).scalar_mul(dx));

    if let Some(wall) = vec::is_hit(position, delta_position, *map) {
        if (stuff::Vector::new(wall[0], wall[1]) - position).len() < delta_position.len() {
            delta_position = stuff::Vector::new(0, 0);
        }
    }

    if window.is_key_down(Key::Left) {
        angle += rotation_speed;
        if angle > 3.14159 * 2.0 {
            angle = angle - 3.14159 * 2.0;
        }
    }
    
    if window.is_key_down(Key::Right) {
        angle -= rotation_speed;
        if angle < -3.14159 * 2.0 {
            angle = angle + 3.14159 * 2.0;
        }
    }

    (angle, position + delta_position)
}