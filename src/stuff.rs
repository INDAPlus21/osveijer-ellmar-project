use std::ops::{Add, Sub, Index, AddAssign, SubAssign};

pub const WIDTH: usize = 500; // window witdth
const FOV: f32 = 3.14159 / 4.0;
const PI: f32 = 3.14159;

#[derive(Clone, Copy)]

pub enum Direction {
	Forward,
	Backward,
	Right,
	Left,
}

#[derive(Copy, Clone)]
struct Vector {
	x: f32,
	y: f32
}

impl Vector {
	fn new(x: f32, y: f32) -> Self {
		Self{x, y}
	}
	
	fn get_x(&self) -> f32 {
		self.x
	}

	fn get_y(&self) -> f32 {
		self.y
	}

	// create vector of unit length representing angle in radians ( 0 = 03:00, pi = 09:00)
	fn from_angle(angle: f32) -> Self {
		Self {
			x: angle.cos(),
			y: angle.sin(),
		}
	}
	
	fn scalar_mul(&self, scalar: f32) -> Self {
		Self {
			x: self.x * scalar,
			y: self.y * scalar
		}
	}
	
	fn scalar_div(&self, scalar: f32) -> Self {
		Self {
			x: self.x / scalar,
			y: self.y / scalar
		}
	}

	fn len(&self) -> f32 {
		(self.x * self.x + self.y * self.y).sqrt()
	}
	
	fn is_hit(player: Self, angle: Self, map: Map) -> Option<[usize; 2]> {
		let mut step: f32 = 0.0;

		loop {
			
			let new_location = player + angle.scalar_mul(step);

			let x = new_location.x.round() as usize;
			let y = new_location.y.round() as usize;
			
			if x > 15 || y > 15 {break;}

			if map[y][x] {
				//println!("{}, {}", x, y);
				return Some([x, y])
			}
			step += 0.1
		}
		None
	}
}

impl Sub for Vector {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y
		}
	}
}

impl Add for Vector {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y
		}
	}
}

impl AddAssign for Vector {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl SubAssign for Vector {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}

#[derive(Clone, Copy)]
pub struct Player {
	pos: Vector,
	angle: f32,
}

impl Player {
	pub fn new(x: f32, y: f32, angle: f32) -> Self {
		let pos = Vector::new(x, y);
		Self {pos, angle}
	}

	/// return x, y, angle
	fn get_pos(&self) -> (f32, f32, f32) {
		(self.pos.get_x(), self.pos.get_y(), self.angle)
	}
	
	pub fn rotate(&mut self, dir: Direction, angle: f32) {
		match dir {
			Direction::Left => {
				self.angle += angle;
				if self.angle > PI * 2.0 {
					self.angle -= PI * 2.0;
				}
			},
			Direction::Right => {
				self.angle -= angle;
				if self.angle < 0.0 {
					self.angle += PI * 2.0;
				}
			}
			_ => {
				panic!("direction of rotation must be left or right");
			}
		}
	}

	pub fn mv(&mut self, dir: Direction) {
		
		// factor with which to divide direction vector with
		let factor = 10.0; // TODO: is this fine?
		match dir {
			Direction::Forward => {
				self.pos += Vector::from_angle(self.angle).scalar_div(factor);
			},
			Direction::Left => {
				self.pos += Vector::from_angle(self.angle + PI / 2.0).scalar_div(factor);
			}
			Direction::Backward => {
				self.pos -= Vector::from_angle(self.angle).scalar_div(factor);
			}
			Direction::Right => {
				self.pos += Vector::from_angle(self.angle - PI / 2.0).scalar_div(factor);
			}
		}
	}
}

#[derive(Clone, Copy)]
pub struct Map {
	map: [[bool; 16]; 16]
}

impl Map {
	pub fn new(arr: [&str; 16]) -> Self {
		Self::make_map(arr)
	}

	fn make_map(arr: [&str; 16]) -> Self {

		let mut map = [[false; 16]; 16];

		for (outer, arr) in arr.iter().enumerate() {
			for (inner, char) in arr.chars().enumerate() {
				if char == '#' {
					map[outer][inner] = true;
				}
			}
		}

		Self {map}
	}
}

impl Index<usize> for Map {
	type Output = [bool; 16];

	fn index(&self, index: usize) -> &Self::Output {
		&self.map[index]
	}
}

pub fn render(player: Player, map: Map) -> [f32; WIDTH] {
	let mut result = [0.0; WIDTH];
	let step = FOV / (WIDTH as f32);

	let (x, y, angle) = player.get_pos();
	let player_vec = Vector::new(x, y);

	let mut angle_current = angle + FOV / 2.0;

	for idx in 0..WIDTH {
		let angle_vec = Vector::from_angle(angle_current);
		if let Some(wall) = Vector::is_hit(player_vec, angle_vec, map) {
			result[idx] = (player_vec - Vector::new(wall[0] as f32, wall[1] as f32)).len();
		} else {
			result[idx] = 100.0;
		}
		angle_current -= step;
	}
	result
}