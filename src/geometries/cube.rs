use geometries::object::Object;

pub struct Cube {
    pub a: f64
}

impl Object for Cube {
    const OBJECT_TYPE: &'static str = "Cube";

    fn get_volume(self: &Cube) -> f64 {
        return self.a.powi(3);
    }

    fn get_surface_area(self: &Cube) -> f64 {
        return self.a.powi(2) * 6.0;
    }
}