mod stuff;
extern crate minifb;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = stuff::WIDTH;
const HEIGHT: usize = WIDTH / 2;
const PI: f32 = 3.14159;

const CLR_FLOOR: u32 = from_u8_rgb(150, 150, 150);
const CLR_SKY: u32 = from_u8_rgb(125, 150, 255);
const CLR_KEY: u32 = from_u8_rgb(200, 200, 20);
const CLR_WALL: u32 = from_u8_rgb(70, 120, 175);
const CLR_GATE: u32 = from_u8_rgb(250, 50, 50);

// code for conversion from rgb values to minifb colors taken from the minifb documentation
const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
	let (r, g, b) = (r as u32, g as u32, b as u32);
	(r << 16) | (g << 8) | b
}

pub fn main() {
	let map_string = [
		"################",
		"#..............#",
		"#.......########",
		"#.....###......#",
		"#.....#E#......#",
		"#.....#$#......#",
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
	
	let mut map = stuff::Map::new(map_string);
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
	let mut player = stuff::Player::new(2.0, 2.0, angle, false);
	let mut exit = false;

	while window.is_open() && !window.is_key_down(Key::Escape) && !exit{
		
		let keys = window.get_keys();

		// arrows and wasd are matched seperately, so that the player can move
		// and look at the same time
		for t in keys {
			match t {
				Key::Left => {
					player.rotate(stuff::Direction::Left, 0.05);
				},
				Key::Right => {
					player.rotate(stuff::Direction::Right, 0.05);
				},
				_ => {()}
			}
			
			let dir: Option<stuff::Direction> = match t {
				Key::W => {
					Some(stuff::Direction::Forward)
				},
				Key::A => {
					Some(stuff::Direction::Left)
				}
				Key::S => {
					Some(stuff::Direction::Backward)
				}
				Key::D => {
					Some(stuff::Direction::Right)
				}
				_ => (None)
			};

			if dir.is_some() {
				exit = player.mv(dir.unwrap(), &mut map);
			}
		}

		println!("{}\n", stuff::minimap(map, player));


		let data = stuff::render(player, map);

		for (idx, pixel) in buffer.iter_mut().enumerate() {
			let column = idx % WIDTH;
			let row = (idx - (idx % HEIGHT)) / HEIGHT;

			let dat = data[column];
			if dat[3] > 0.0 && dat[4] == 1.0 && HEIGHT as f32 / 2.0 - (((1.0/dat[3]).atan() * HEIGHT as f32 * 2.0)/stuff::FOV) < row as f32 && HEIGHT as f32 / 2.0 + (((1.0/dat[3]).atan() * HEIGHT as f32 * 2.0)/stuff::FOV) > row as f32 {
				*pixel = CLR_GATE;
			} else if dat[2] > 0.0 && HEIGHT as f32 / 2.0 - (((0.2/dat[2]).atan() * HEIGHT as f32 * 2.0)/stuff::FOV) < row as f32 && HEIGHT as f32 / 2.0 + (((0.2/dat[2]).atan() * HEIGHT as f32 * 2.0)/stuff::FOV) > row as f32 {
				*pixel = CLR_KEY;
			} else if dat[0] < 15.0 && HEIGHT as f32 / 2.0 - (((1.0/dat[0]).atan() * HEIGHT as f32 * 2.0)/stuff::FOV) < row as f32 && HEIGHT as f32 / 2.0 + (((1.0/dat[0]).atan() * HEIGHT as f32 * 2.0)/stuff::FOV) > row as f32 {
				*pixel = dat[1] as u32 * 2000 + CLR_WALL;
			} else if HEIGHT / 2 < row {
				*pixel = CLR_FLOOR;
			} else {
				*pixel = CLR_SKY;
			}
		}

		// We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
		window
			.update_with_buffer(&buffer, WIDTH, HEIGHT)
			.unwrap();
	}
}

