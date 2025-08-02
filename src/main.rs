const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = 400;

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let red: f32 = i as f32 / (IMAGE_HEIGHT - 1) as f32;
            let green: f32 = j as f32 / (IMAGE_WIDTH - 1) as f32;
            let blue: f32 = 0.0;

            let red_out: i32 = (red * 255.0) as i32;
            let green_out: i32 = (green * 255.0) as i32;
            let blue_out: i32 = (blue * 255.0) as i32;

            println!("{} {} {}", red_out, green_out, blue_out);
        }
    }
}

#[derive(Copy, Debug, Default, Clone, PartialEq)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Add for Vec3 {
    type Output = Self;
}
