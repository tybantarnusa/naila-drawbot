mod imager;
use image::{Rgb, DynamicImage, GenericImageView };
use winapi::um::winuser::{ mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, SetCursorPos };
// use winapi::shared::windef::POINT;

use std::thread;
use std::time::Duration;

// fn main() {
//     loop {
//         let mut cursor_pos = POINT { x: 0, y: 0 };
//         unsafe { GetCursorPos(&mut cursor_pos) };
//         println!("Mouse position: ({}, {})", cursor_pos.x, cursor_pos.y);
//     }
// }

fn main() {
    let init_position: (i32, i32) = (25, 170);

    let img: DynamicImage = image::open("kasumi.png").unwrap();
    let pixels: Vec<Rgb<u8>> = imager::get_pixels(&img);
    let dimension: (u32, u32) = img.dimensions();

    unsafe {
        SetCursorPos(635, 70);
        mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
        thread::sleep(Duration::from_millis(10));
        mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
    }

    let mut i = 0;
    let mut last_color: (i32, i32) = (-100, -100);
    let mut last_y: i32 = -100;
    for y in 0..dimension.1 as i32 {
        for x in 0..dimension.0 as i32 {

            let closest_color: (i32, i32) = imager::get_closest_color_from_palette(&pixels[i]);
            
            if last_color.0 != closest_color.0 || last_color.1 != closest_color.1 {
                last_color = closest_color;
                unsafe {
                    println!("x: {}, y: {}", closest_color.0, closest_color.1);
                    mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                    thread::sleep(Duration::from_millis(1));
                    SetCursorPos(closest_color.0, closest_color.1);
                    thread::sleep(Duration::from_millis(1));
                    mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                    thread::sleep(Duration::from_millis(1));
                    mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                    thread::sleep(Duration::from_millis(1));
                    SetCursorPos(init_position.0 + x, init_position.1 + y);
                    thread::sleep(Duration::from_millis(1));
                    mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                };
            } else {
                unsafe {
                    if last_y != init_position.1 + y {
                        mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                    }
                    SetCursorPos(init_position.0 + x, init_position.1 + y);
                    if last_y != init_position.1 + y {
                        thread::sleep(Duration::from_millis(1));
                        mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                        last_y = init_position.1 + y;
                    }
                }
            }
            i = i + 1;
        }
    }

    unsafe {
        mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
    }
}
