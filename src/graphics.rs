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
		"#.............@#",
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
		
		let floor = from_u8_rgb(150, 150, 150);
		let sky = from_u8_rgb(125, 150, 255);
		let key = from_u8_rgb(200, 200, 20);

		for (idx, pixel) in buffer.iter_mut().enumerate() {
			let column = idx % WIDTH;
			let row = (idx - (idx % HEIGHT)) / HEIGHT;

			let dat = data[column];
			if dat[2] != 0.0 && HEIGHT as f32 / 2.0 - (((1.0/dat[2]).atan() * HEIGHT as f32 * 4.0)/(2.0 * 3.14159)) < row as f32 && HEIGHT as f32 / 2.0 + (((1.0/dat[2]).atan() * HEIGHT as f32 * 4.0)/(2.0 * 3.14159)) > row as f32 {
				*pixel = key;
			} else if dat[0] < 15.0 && HEIGHT as f32 / 2.0 - (((3.0/dat[0]).atan() * HEIGHT as f32 * 4.0)/(2.0 * 3.14159)) < row as f32 && HEIGHT as f32 / 2.0 + (((5.0/dat[0]).atan() * HEIGHT as f32 * 4.0)/(2.0 * 3.14159)) > row as f32 {
				*pixel = dat[1].round() as u32 * 100 + 16000000;
			} else if HEIGHT / 2 < row {
				*pixel = floor;
			} else {
				*pixel = sky;
			}
		}

		// We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
		window
			.update_with_buffer(&buffer, WIDTH, HEIGHT)
			.unwrap();
	}
}

// code for conversion from rgb values to minifb colors taken from the minifb documentation
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}