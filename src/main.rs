mod imager;
mod mspaint;
mod config;

use image::{self, GenericImageView};
use mspaint::PaintPosition;
use ulid::Ulid;
use std::fs;
use std::thread;
use std::time::{Duration, Instant};
use winapi::um::winuser;

use crate::config::ConfigData;
use crate::config::Config;

fn main() {
    let config_str = fs::read_to_string("config.toml").expect("Cannot find config file.");
    let config: Config;
    {
        let config_data: ConfigData = toml::from_str(&config_str).expect("Invalid config file.");
        config = config_data.config;
    }

    const DELAY: u64 = 1;
    let image_filename = format!("{}\\drawing\\{}", &config.naila_dir, &config.image_filename);

    loop {
        let mut i: usize = 0;

        let img: image::DynamicImage;
        loop {
            println!("Looking for image...");
            img = match image::open(&image_filename) {
                Ok(loaded_img) => loaded_img,
                Err(_) => continue,
            };
            break;
        }
        let pixels: Vec<image::Rgb<u8>> = imager::get_pixels(&img);
        let dimension: (u32, u32) = img.dimensions();

        let mut sorted_color_vec: Vec<(u32, u32, image::Rgb<u8>)> = Vec::new();
        {
            let mut result_img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
                image::RgbImage::new(dimension.0, dimension.1);
            for y in 0..dimension.1 {
                for x in 0..dimension.0 {
                    let (_, closest_color): (usize, image::Rgb<u8>) =
                        imager::get_closest_color(&pixels[i]);
                    result_img.put_pixel(x, y, closest_color);
                    if !imager::is_white(&closest_color) {
                        sorted_color_vec.push((x, y, closest_color));
                    }
                    i += 1;
                }
            }
            result_img.save("result.png").unwrap();
        }

        sorted_color_vec.sort_by(
            |a: &(u32, u32, image::Rgb<u8>), b: &(u32, u32, image::Rgb<u8>)| {
                imager::compare_color(&a.2, &b.2).unwrap()
            },
        );

        let mut result_color_vec: Vec<(u32, u32, image::Rgb<u8>)> = Vec::new();
        for y in 0..dimension.1 {
            for x in 0..dimension.0 {
                result_color_vec.push((x, y, image::Rgb([255, 255, 255])));
            }
        }
        result_color_vec.extend(sorted_color_vec);

        let paint_window: PaintPosition = PaintPosition::new();
        let initial_canvas_pos: &(i32, i32) = &paint_window.initial_canvas_pos;
        let initial_palette_pos: &(i32, i32) = &paint_window.initial_palette_pos;

        unsafe {
            winuser::SetCursorPos(initial_palette_pos.0, initial_palette_pos.1);
            winuser::mouse_event(winuser::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
            thread::sleep(Duration::from_millis(DELAY));
            winuser::mouse_event(winuser::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
        }

        let mut last_color: (i32, i32) = (i32::MIN, i32::MIN);
        let mut last_x: i32 = result_color_vec[0].0 as i32 + initial_canvas_pos.0;
        let mut last_y: i32 = result_color_vec[0].1 as i32 + initial_canvas_pos.1;

        let start_time: Instant = Instant::now();
        println!("Drawing... (Press ESC to stop)");
        for result_color in result_color_vec.iter() {
            if unsafe { winuser::GetAsyncKeyState(winuser::VK_ESCAPE) } != 0 {
                break;
            }

            let x: i32 = result_color.0 as i32;
            let y: i32 = result_color.1 as i32;

            //TODO: improve so don't call this again
            let closest_color: (i32, i32) =
                imager::get_closest_color_from_palette(&result_color.2, initial_palette_pos);

            if last_color.0 != closest_color.0 || last_color.1 != closest_color.1 {
                last_color = closest_color;
                unsafe {
                    winuser::mouse_event(winuser::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                    thread::sleep(Duration::from_millis(DELAY));
                    winuser::SetCursorPos(closest_color.0, closest_color.1);
                    thread::sleep(Duration::from_millis(DELAY));
                    for _ in 0..3 {
                        winuser::mouse_event(winuser::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                        thread::sleep(Duration::from_millis(DELAY));
                        winuser::mouse_event(winuser::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                        thread::sleep(Duration::from_millis(DELAY));
                    }
                    winuser::SetCursorPos(initial_canvas_pos.0 + x, initial_canvas_pos.1 + y);
                    thread::sleep(Duration::from_millis(DELAY));
                    winuser::mouse_event(winuser::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                }
            } else {
                unsafe {
                    if last_y != initial_canvas_pos.1 + y || last_x != initial_canvas_pos.0 + x - 1
                    {
                        winuser::mouse_event(winuser::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                    }
                    winuser::SetCursorPos(initial_canvas_pos.0 + x, initial_canvas_pos.1 + y);
                    if last_y != initial_canvas_pos.1 + y || last_x != initial_canvas_pos.0 + x - 1
                    {
                        thread::sleep(Duration::from_millis(DELAY));
                        winuser::mouse_event(winuser::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                        last_y = initial_canvas_pos.1 + y;
                    }
                }
            }

            last_x = initial_canvas_pos.0 + x;
        }

        unsafe {
            winuser::mouse_event(winuser::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
        }

        let drawing_duration: Duration = start_time.elapsed();
        println!(
            "DONE! Drawing duration is {} minutes {} seconds.",
            drawing_duration.as_secs() / 60,
            drawing_duration.as_secs() % 60
        );

        let tunnel_path = format!("{}\\drawing\\{}", &config.naila_dir, &config.tunnel_filename);
        let prompt = match fs::read_to_string(&tunnel_path) {
            Ok(txt) => txt,
            Err(_) => {
                let ulid = Ulid::new();
                ulid.to_string()
            }
        };

        if fs::rename(&image_filename, format!("{}\\drawing\\{}.png", &config.naila_dir, prompt)).is_err() {
            println!("Cannot rename image.");
        }
    }
}
