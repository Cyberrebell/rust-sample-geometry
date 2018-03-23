use std::f64::consts::PI;
use geometries::object::Object;

pub struct Sphere {
    pub d: f64
}

impl Object for Sphere {
    const OBJECT_TYPE: &'static str = "Sphere";

    fn get_volume(self: &Sphere) -> f64 {
        return (PI / 6.0) * self.d.powi(3);
    }

    fn get_surface_area(self: &Sphere) -> f64 {
        return PI * self.d.powi(2);
    }
}