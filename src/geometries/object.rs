pub trait Object {
    const OBJECT_TYPE: &'static str = "object";

    fn get_volume(&self) -> f64;

    fn get_surface_area(&self) -> f64;

    fn print(&self) {
        println!("My {} volume is {}", Self::OBJECT_TYPE, self.get_volume());
        println!("My {} surface area is {}", Self::OBJECT_TYPE, self.get_surface_area());
    }
}