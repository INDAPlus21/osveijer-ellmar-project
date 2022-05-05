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

	let angle = PI / 2.0;
	let mut player = stuff::Player::new(2.0, 2.0, angle);

	while window.is_open() && !window.is_key_down(Key::Escape) {
		
		let keys = window.get_keys();

		// arrows and wasd are matched seperately, so that the player can move
		// and look at the same time
		for t in keys {
			match t {
				Key::Left => {
					player.rotate(stuff::Direction::Left, 0.1); // TODO: is angle fine?
				},
				Key::Right => {
					player.rotate(stuff::Direction::Right, 0.1);
				},
				_ => {()}
			}
			
			match t {
				Key::W => {
					player.mv(stuff::Direction::Forward, map);
				},
				Key::A => {
					player.mv(stuff::Direction::Left, map);
				}
				Key::S => {
					player.mv(stuff::Direction::Backward, map);
				}
				Key::D => {
					player.mv(stuff::Direction::Right, map);
				}
				_ => ()
			}
		}

		let data = stuff::render(player, map);

		for (idx, pixel) in buffer.iter_mut().enumerate() {
			let column = idx % WIDTH;
			let row = (idx - (idx % HEIGHT)) / HEIGHT;

			let dist = data[column];

			if dist < 6.0 && ((dist / 3.0) * (HEIGHT / 2) as f32) < row as f32 {
				*pixel = dist.round() as u32 * 100 + 100000;
			} else {
				*pixel = 0;
			}
		}

		// We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
		window
			.update_with_buffer(&buffer, WIDTH, HEIGHT)
			.unwrap();
	}
}