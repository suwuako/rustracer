use crate::vectors::Vec3;
pub type Colour = Vec3;


impl Colour {
    pub fn print_colour(&self) -> () {
        let ir: i32 = self.x().floor() as i32;
        let ig: i32 = self.y().floor() as i32;
        let ib: i32 = self.z().floor() as i32;
        println!("{} {} {}", ir, ig, ib);
    }
}
