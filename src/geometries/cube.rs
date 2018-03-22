pub struct Cube {
    pub a: f64
}

impl Cube {
    pub fn get_volume(self: Cube) -> f64 {
        return self.a * self.a * self.a;
    }
}