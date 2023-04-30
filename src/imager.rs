use image::{self, DynamicImage, ImageBuffer, Rgb};

pub fn get_pixels(img: &DynamicImage) -> Vec<Rgb<u8>> {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = img.to_rgb8();
    let mut pixels: Vec<Rgb<u8>> = Vec::new();
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel: &Rgb<u8> = img.get_pixel(x, y);
            pixels.push(*pixel)
        }
    }

    pixels
}

pub fn get_closest_color_from_palette(
    color: &Rgb<u8>,
    initial_palette_pos: &(i32, i32),
) -> (i32, i32) {
    let palette: [Rgb<u8>; 20] = [
        Rgb([0, 0, 0]),
        Rgb([127, 127, 127]),
        Rgb([138, 8, 26]),
        Rgb([240, 26, 36]),
        Rgb([254, 128, 41]),
        Rgb([253, 242, 1]),
        Rgb([35, 179, 78]),
        Rgb([3, 164, 234]),
        Rgb([63, 72, 203]),
        Rgb([165, 72, 163]),
        Rgb([255, 255, 255]),
        Rgb([196, 196, 196]),
        Rgb([183, 124, 87]),
        Rgb([255, 174, 203]),
        Rgb([253, 201, 16]),
        Rgb([238, 225, 175]),
        Rgb([179, 230, 30]),
        Rgb([154, 217, 234]),
        Rgb([115, 145, 195]),
        Rgb([199, 191, 230]),
    ];

    let mut distance_vector: Vec<(usize, f64)> = Vec::new();
    for i in 0..palette.len() {
        let color2: Rgb<u8> = palette[i];

        let color1_float: [f64; 3] = [color[0] as f64, color[1] as f64, color[2] as f64];
        let color2_float: [f64; 3] = [color2[0] as f64, color2[1] as f64, color2[2] as f64];

        let r: f64 = (color1_float[0] + color2_float[0]) / 2.0;
        let distance: f64 = (((color2_float[0] - color1_float[0]) * (2.0 + (r / 256.0))).powf(2.0)
            + ((color2_float[1] - color1_float[1]) * 4.0).powf(2.0)
            + ((color2_float[2] - color1_float[2]) * (2.0 + ((256.0 - r) / 256.0))).powf(2.0))
        .sqrt();
        distance_vector.push((i, distance));
    }

    distance_vector.sort_by(|a: &(usize, f64), b: &(usize, f64)| a.1.partial_cmp(&b.1).unwrap());
    let closest_idx: usize = distance_vector[0].0;
    let mouse_pos: (i32, i32) = (
        initial_palette_pos.0 + (closest_idx as i32 % 10 * 22),
        (initial_palette_pos.1 + (closest_idx as i32 / 10 * 22)),
    );

    mouse_pos
}
