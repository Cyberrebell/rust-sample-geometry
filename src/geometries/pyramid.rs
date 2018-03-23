use geometries::object::Object;

pub struct Pyramid {
    pub a: f64,
    pub h: f64
}

impl Object for Pyramid {
    const OBJECT_TYPE: &'static str = "Pyramid";

    fn get_volume(self: &Pyramid) -> f64 {
        return self.a.powi(2) * self.h / 3.0;
    }

    fn get_surface_area(self: &Pyramid) -> f64 {
        return self.a.powi(2) + (self.a * (4.0 * self.h.powi(2) + self.a.powi(2)).sqrt());
    }
}