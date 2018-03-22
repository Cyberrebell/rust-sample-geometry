mod geometries;
pub use geometries::cube;

fn main() {
	let my_cube = geometries::cube::Cube { a: 16.7 };

	println!("My cube volume is {}", my_cube.get_volume());
}