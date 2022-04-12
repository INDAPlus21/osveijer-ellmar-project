use crate::stuff;

extern crate minifb;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = stuff::WIDTH;
const HEIGHT: usize = WIDTH / 2;

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
		"Test - ESC to exit",
		WIDTH,
		HEIGHT,
		WindowOptions::default(),
	)
	.unwrap_or_else(|e| {
		panic!("{}", e);
	});

	// Limit to max ~60 fps update rate
	window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

	let mut angle = 0.0;

	while window.is_open() && !window.is_key_down(Key::Escape) {
		
		if window.is_key_down(Key::Left) {
			angle += 0.05;
			if angle > 3.14159 * 2.0 {
				angle = angle - 3.14159 * 2.0;
			}
		}
		
		if window.is_key_down(Key::Right) {
			angle -= 0.05;
			if angle < -3.14159 * 2.0 {
				angle = angle + 3.14159 * 2.0;
			}
		}

		let mut idx = 0; // column
		for i in buffer.iter_mut() {

			let player = stuff::Player::new(2.0, 2.0, angle);
			let data = stuff::render(player, map);

			let dist = data[idx % WIDTH].round() as u32 * 1000;

			*i = dist; // write something more funny here!
			idx += 1;
		}

		// We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
		window
			.update_with_buffer(&buffer, WIDTH, HEIGHT)
			.unwrap();
	}
}