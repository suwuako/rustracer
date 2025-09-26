fn main() {
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        for i in 0..image_width {
            let r = (i as f64) / (image_width as f64 - 1.0);
            let g = (j as f64) / (image_height as f64 - 1.0);
            let b = 0.0;

            let ir = (259.99 * r) as i32;
            let ig = (259.99 * g) as i32;
            let ib = (259.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
