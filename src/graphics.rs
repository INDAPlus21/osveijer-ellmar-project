use crate::stuff;

extern crate minifb;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = stuff::WIDTH;
const HEIGHT: usize = WIDTH / 2;
const PI: f32 = 3.14159;

pub fn render() {
	let map_string = [
		"#########.......",
		"#...............",
		"#.......########",
		"#..............#",
		"#......##......#",
		"#......##......#",
		"#..............#",
		"###............#",
		"##.............#",
		"#......####..###",
		"#......#.......#",
		"#......#.......#",
		"#..............#",
		"#......#########",
		"#..............#",
		"################",
	];
	
	let map = stuff::Map::new(map_string);
	
	let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

	let mut window = Window::new(
		"d00m - ESC to exit",
		WIDTH,
		HEIGHT,
		WindowOptions::default(),
	)
	.unwrap_or_else(|e| {
		panic!("{}", e);
	});

	// Limit to max ~60 fps update rate
	window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

	let mut angle = PI / 2.0;
	let mut position = stuff::Vector::new(2.0, 2.0);

	while window.is_open() && !window.is_key_down(Key::Escape) {
		
		let keys = window.get_keys();

		// arrows and wasd are matched seperately, so that the player can move
		// and look at the same time
		// TODO: functions for movement!!
		for t in keys {
			match t {
				Key::Left => {
					angle += 0.05;
					if angle > 3.14159 * 2.0 {
						angle = angle - 3.14159 * 2.0;
					}
				},
				Key::Right => {
					angle -= 0.05;
					if angle < -3.14159 * 2.0 {
						angle = angle + 3.14159 * 2.0;
					}
				},
				_ => {()}
			}
			
			match t {
				Key::W => {
					position = position + stuff::Vector::from_angle(angle).scalar_div(10.0);
				},
				Key::A => {
					position = position + stuff::Vector::from_angle(angle + PI / 2.0).scalar_div(10.0);
				}
				Key::S => {
					position = position - stuff::Vector::from_angle(angle).scalar_div(10.0);
				}
				Key::D => {
					position = position + stuff::Vector::from_angle(angle - PI / 2.0).scalar_div(10.0);
				}
				_ => ()
			}
		}

		let player = stuff::Player::new(position.get_x(), position.get_y(), angle);
		let data = stuff::render(player, map);

		for (idx, pixel) in buffer.iter_mut().enumerate() {
			let column = idx % WIDTH;
			let row = (idx - (idx % HEIGHT)) / HEIGHT;

			let dist = data[column];

			if row < dist as usize * 50 {
				*pixel = 0;
			} else {
				*pixel = dist.round() as u32 * 100;
			}

		}

		// We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
		window
			.update_with_buffer(&buffer, WIDTH, HEIGHT)
			.unwrap();
	}
}