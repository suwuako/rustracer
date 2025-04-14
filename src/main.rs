pub mod vectors;
pub mod colour;

use vectors::Vec3;
use colour::Colour;


fn render(dim: &Vec3) -> () {
    let height: i32 = dim.y() as i32;
    let width: i32 = dim.x() as i32;
    println!("P3\n{} {}\n255\n", width, height);

    for x in 0..height {
        eprintln!("Scanlines remaining: {}", x);
        for y in 0..width {
            // interp between 0 - 255
            let r: f64 = (y as f64 / (width - 1) as f64) * 255.0;
            let g: f64 = (x as f64 / (height - 1) as f64) * 255.0;
            let b: f64 = 255.0;

            let pixel = Colour::new(r, g, b);
            pixel.print_colour();
        }
    }
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let width: i32 = 800;
    let height: i32 = (1.0 / aspect_ratio * (width as f64)).floor() as i32;
    let image_dim: Vec3 = Vec3::new(width as f64, height as f64, 0.0);

    render(&image_dim);
    image_dim.print();
}
