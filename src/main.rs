mod geometries;
use geometries::object::Object;

fn main() {
	let my_cube = geometries::cube::Cube { a: 16.7 };
	my_cube.print();

	let my_sphere = geometries::sphere::Sphere { d: 16.7 };
	my_sphere.print();

	let my_pyramid = geometries::pyramid::Pyramid { a: 16.7, h: 10.8 };
	my_pyramid.print();
}