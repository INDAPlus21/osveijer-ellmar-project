use std::ops::{Add, Sub, Index, AddAssign, SubAssign, IndexMut};

pub const WIDTH: usize = 1000; // window witdth
pub const FOV: f32 = 3.14159 / 4.0;
const PI: f32 = 3.14159;

/// player / camera movement
#[derive(Clone, Copy)]
pub enum Direction {
	Forward,
	Backward,
	Right,
	Left,
}

/// elements used in building maps
#[derive(Clone, Copy, PartialEq)]
pub enum MapElem {
	Wall,
	Key,
	GateClosed,
	GateOpened,
	Void,
	Exit,
}

/// # 2d vectors for used for graphics / coordinates
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

	/// create vector of unit length representing angle in radians
	/// * 0 rad => pointing positive x
	/// * (pi / 2) rad => pointing negative y
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

	fn dot(a: Self, b: Self) -> f32 {
		a.x * b.x + a.y * b.y
	}
	
	/// Determine if a ray shot from a coordinate: player, with
	/// an angle: angle hits anything on the map.
	/// 
	/// return: [x, y, wall_orientation?, key, gate, onbar?]
	fn is_hit(player: Self, angle: Self, map: Map) -> Option<[f32; 6]> {
		let step: f32 = 0.01; // length increment size
		let mut length: f32 = 0.0; // length of ray
		let mut key: f32 = 0.0;
		let mut gate: f32 = 0.0;
		let mut onbar: f32 = 0.0;

		loop {
			
			let new_location = player + angle.scalar_mul(length);

			let x = new_location.x.round() as usize;
			let y = new_location.y.round() as usize;
			
			if x > 15 || y > 15 {break;}

			match map[y][x] {
				MapElem::Wall => {
					let wallvec = Vector::new(x as f32,y as f32);
					let dir = player - wallvec;

					let (x_1, x_2): (f32, f32) = if dir.x > 0.0 {
						(1.0, 0.5)
					} else {
						(-1.0, -0.5)
					};

					let (y_1, y_2): (f32, f32) = if dir.y > 0.0 {
						(1.0, 0.5)
					} else {
						(-1.0, -0.5)
					};

					if let Some(hitdata) = Vector::intersect(Vector::new(0.0,y_1), wallvec + Vector::new(0.0, y_2), angle, player, MapElem::Wall) {
						return Some([hitdata.0.x, hitdata.0.y, hitdata.1, key, gate, onbar]);
					} 
					else if let Some(hitdata) = Vector::intersect(Vector::new(x_1, 0.0), wallvec + Vector::new(x_2, 0.0), angle, player, MapElem::Wall) {
						return Some([hitdata.0.x, hitdata.0.y, hitdata.1, key, gate, onbar]);
					}
				},

				MapElem::Key => {
					if key == 0.0 {
						let keypos = Vector::new(x as f32, y as f32);
						if let Some(hitdata) = Vector::intersect(player-keypos,  keypos, angle, player, MapElem::Key) {
							key = (player - Vector::new(hitdata.0.x, hitdata.0.y)).len();
						} else {
							key = -1.0;
						}
					}
				},

				MapElem::GateClosed => {
					if gate == 0.0 {
						let gatepos = Vector::new(x as f32, y as f32);
						let dir = player - gatepos;

						let (x_1, x_2): (f32, f32) = if dir.x > 0.0 {
							(1.0, 0.4)
						} else {
							(-1.0, -0.4)
						};

						let (y_1, y_2): (f32, f32) = if dir.y > 0.0 {
							(1.0, 0.4)
						} else {
							(-1.0, -0.4)
						};

						if let Some(hitdata) = Vector::intersect(Vector::new(0.0, y_1), gatepos + Vector::new(0.0, y_2), angle, player, MapElem::GateClosed) {
							gate = (player - Vector::new(hitdata.0.x, hitdata.0.y)).len();
							onbar = hitdata.1;
							continue;
						}

						if let Some(hitdata) = Vector::intersect(Vector::new(x_1,0.0), gatepos + Vector::new(x_2, 0.0), angle, player, MapElem::GateClosed) {
							gate = (player - Vector::new(hitdata.0.x, hitdata.0.y)).len();
							onbar = hitdata.1;
							continue;
						}

						gate = -1.0;
					}
				},

				MapElem::GateOpened => {
					if gate == 0.0 {
						let gatepos = Vector::new(x as f32, y as f32);
						let dir = player - gatepos;

						let (x_1, x_2): (f32, f32) = if dir.x > 0.0 {
							(1.0, 0.5)
						} else {
							(-1.0, -0.5)
						};

						let (y_1, y_2): (f32, f32) = if dir.y > 0.0 {
							(1.0, 0.5)
						} else {
							(-1.0, -0.5)
						};

						if let Some(hitdata) = Vector::intersect(Vector::new(0.0,y_1), gatepos + Vector::new(0.0, y_2), angle, player, MapElem::GateOpened) {
							gate = (player - Vector::new(hitdata.0.x, hitdata.0.y)).len();
							onbar = 1.0;
							continue;
						} else if let Some(hitdata) = Vector::intersect(Vector::new(x_1, 0.0), gatepos + Vector::new(x_2, 0.0), angle, player, MapElem::GateOpened) {
							gate = (player - Vector::new(hitdata.0.x, hitdata.0.y)).len();
							onbar = 1.0;
							continue;
						}
						gate = -1.0;
					}
				},
				
				_ => {
					if (x, y) == (0, 0) {break;}
				},
			}	
			length += step
		}
		None
	}

	fn intersect(normal: Self, center: Self, angle: Self, player: Self, elem: MapElem) -> Option<(Self, f32)> {
		let denominator = Vector::dot(normal, angle);

		if denominator != 0.0 {
			let t = (Vector::dot(normal, center) - Vector::dot(normal, player)) / denominator;
			if t > 0.0 {
				let p = player + angle.scalar_mul(t);
				match elem {
					MapElem::Wall => {
						if (p - center).len() <= 0.5 {
							return Some((p, normal.x.abs()));
						}
					},
					MapElem::Key => {
						if (p - center).len() <= 0.1 {
							return Some((p, normal.x.abs()));
						}
					},
					MapElem::GateClosed => {
						if (p - center).len() <= 0.4 {
							if ((p - center).len() * 10.0 + 0.5).floor() as usize % 2 == 0 {
								return Some((p, 1.0));
							} else {
								return Some((p, 0.0));
							}
						}
					},
					MapElem::GateOpened => {
						if (p - center).len() >= 0.3 && (p - center).len() <= 0.4 {
							return Some((p, normal.x.abs()));
						}
					},
					_ => ()
				}
			}
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

/// # Reperesentation of the player
/// 
/// **fields:**
/// * pos: x, and y coordinates, represented as Vector
/// * angle: camera angle in radians. 0 rad => pointing to positive x, pi / 2 rad => pointing to negative y
/// * has_key: used to determine if player has obtained MapElem::Key
#[derive(Clone, Copy)]
pub struct Player {
	pos: Vector,
	angle: f32,
	has_key: bool
}

impl Player {
	/// angle is in radians (0 => pointing positive x, pi/2 => negative y)
	pub fn new(x: f32, y: f32, angle: f32, has_key: bool) -> Self {
		let pos = Vector::new(x, y);
		Self {pos, angle, has_key}
	}

	/// return x, y, angle
	fn get_pos(&self) -> (f32, f32, f32) {
		(self.pos.get_x(), self.pos.get_y(), self.angle)
	}
	
	/// rotate the player
	/// **arg:s**
	/// * dir: Direction::Left or Direction::Right only
	/// * angle: radians (0 => pointing positive x, pi/2 => negative y)
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

	/// Move the player by a small distance
	/// return true, if the game is over
	pub fn mv(&mut self, dir: Direction, map: &mut Map) -> bool {
		
		// factor with which to divide direction vector with
		let factor = 16.0;
		// movement Vector
		let mut movement: Vector = Vector::new(0.0,0.0);
		// get movement
		match dir {
			Direction::Forward => {
				movement += Vector::from_angle(self.angle).scalar_div(factor);
			},
			Direction::Left => {
				movement += Vector::from_angle(self.angle + PI / 2.0).scalar_div(factor);
			}
			Direction::Backward => {
				movement -= Vector::from_angle(self.angle).scalar_div(factor);
			}
			Direction::Right => {
				movement += Vector::from_angle(self.angle - PI / 2.0).scalar_div(factor);
			}
		}
		// make sure not going through walls
		if let Some(wall) = Vector::is_hit(self.pos, movement, *map) {
			if (self.pos - Vector::new(wall[0] as f32, wall[1] as f32)).len() < movement.len() {
				movement = movement.scalar_div(movement.len()).scalar_mul((self.pos - Vector::new(wall[0] as f32, wall[1] as f32)).len()-0.05);
			} else if wall[4] > 0.0 && wall[4] < movement.len() {
				movement = movement.scalar_div(movement.len()).scalar_mul(wall[4] - 0.05);
			}
		}

		let x_round = self.pos.get_x() as usize;
		let y_round = self.pos.get_y() as usize;
		
		match map[y_round][x_round] {
			MapElem::Key => {
				map[y_round][x_round] = MapElem::Void;
				self.has_key = true;
			},
			MapElem::GateClosed => {
				if self.has_key {
					map[y_round][x_round] = MapElem::GateOpened;
				}
			},
			MapElem::Exit => {
				return true;
			}
			
			_ => (),
		}
		self.pos += movement; 
		false
	}
}

/// # Struct representing map
/// * the map is made from Mapelem.
/// * the map has a size of 16x16
#[derive(Clone, Copy)]
pub struct Map {
	map: [[MapElem; 16]; 16]
}

impl Map {
	/// Create map from string representation
	/// * '.' => MapElem::Void
	/// * '#' => MapElem::Wall
	/// * '@' => MapElem::Key
	/// * '€' => MapElem::GateOpened
	/// * '$' => MapElem::GateClosed
	pub fn new(arr: [&str; 16]) -> Self {
		Self::make_map(arr)
	}

	fn make_map(arr: [&str; 16]) -> Self {

		let mut map = [[MapElem::Void; 16]; 16];

		for (outer, arr) in arr.iter().enumerate() {
			for (inner, char) in arr.chars().enumerate() {
				map[outer][inner] = match char {
					'#' => MapElem::Wall,
					'@' => MapElem::Key,
					'$' => MapElem::GateClosed,
					'€' => MapElem::GateOpened,
					'E' => MapElem::Exit,
					_ => MapElem::Void
				};
			}
		}
		Self {map}
	}
}

impl Index<usize> for Map {
	type Output = [MapElem; 16];

	fn index(&self, index: usize) -> &Self::Output {
		&self.map[index]
	}
}

impl IndexMut<usize> for Map {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		return self.map.index_mut(index);
	}
}

/// "raytracer"
/// 
/// return: [[distance, wall-orientation?, key, gate, onbar?]; WIDTH]
pub fn render(player: Player, map: Map) -> [[f32;5]; WIDTH] {
	let mut result = [[0.0;5]; WIDTH];
	let step = FOV / (WIDTH as f32);

	let (x, y, angle) = player.get_pos();
	let player_vec = Vector::new(x, y);

	let mut angle_current = angle + FOV / 2.0;

	for idx in 0..WIDTH {
		let angle_vec = Vector::from_angle(angle_current);
		if let Some(hit) = Vector::is_hit(player_vec, angle_vec, map) {
			result[idx] = [(player_vec - Vector::new(hit[0], hit[1])).len(), hit[2], hit[3], hit[4], hit[5]];
		} else {
			result[idx] = [100.0,0.0,0.0,0.0,0.0];
		}
		angle_current -= step;
	}
	result
}


	/// Create string representation from map, and draw player
	/// * MapElem::Void => '.'
	/// * MapElem::Wall => '#'
	/// * MapElem::Key => '@'
	/// * MapElem::GateOpened => '€'
	/// * MapElem::GateClosed => '$'
	/// * Player => '8'
pub fn minimap(map: Map, player: Player) -> String {
	let (x, y, _) = player.get_pos();
	let x: usize = x.round() as usize;
	let y: usize = y.round() as usize;

	let mut buf = String::new();
	
	for row_idx in 0..16 {
		for col_idx in 0..16 {
			if (row_idx, col_idx) == (y, x) {
				buf.push('8');
			} else {
				buf.push(match map[row_idx][col_idx]{
					MapElem::Wall => '#',
					MapElem::Key => '@',
					MapElem::Void => '.',
					MapElem::GateClosed => '$',
					MapElem::GateOpened => '€',
					MapElem::Exit => 'E',
				})
			}
			
		}
		buf.push('\n')
	}
	buf
}