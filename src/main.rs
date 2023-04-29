mod imager;
mod mspaint;

use image::{ self, GenericImageView };
use mspaint::PaintPosition;
use winapi::um::winuser;
use std::thread;
use std::time::Duration;

fn main() {
    let paint_window: PaintPosition = PaintPosition::new();
    let initial_canvas_pos: &(i32, i32) = &paint_window.initial_canvas_pos;
    let initial_palette_pos: &(i32, i32) = &paint_window.initial_palette_pos;

    let img: image::DynamicImage = image::open("kasumi.png").unwrap();
    let pixels: Vec<image::Rgb<u8>> = imager::get_pixels(&img);
    let dimension: (u32, u32) = img.dimensions();

    unsafe {
        winuser::SetCursorPos(635, 70);
        winuser::mouse_event(winuser::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
        thread::sleep(Duration::from_millis(1));
        winuser::mouse_event(winuser::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
    }

    let mut i: usize = 0;
    let mut last_color: (i32, i32) = (-100, -100);
    let mut last_y: i32 = -100;

    println!("Drawing... (Press ESC to stop)");
    'draw_loop: for y in 0..dimension.1 as i32 {
        for x in 0..dimension.0 as i32 {
            if unsafe { winuser::GetAsyncKeyState(winuser::VK_ESCAPE) } != 0 {
                break 'draw_loop;
            }

            let closest_color: (i32, i32) = imager::get_closest_color_from_palette(&pixels[i], &initial_palette_pos);
            
            if last_color.0 != closest_color.0 || last_color.1 != closest_color.1 {
                last_color = closest_color;
                unsafe {
                    winuser::mouse_event(winuser::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                    thread::sleep(Duration::from_millis(1));
                    winuser::SetCursorPos(closest_color.0, closest_color.1);
                    thread::sleep(Duration::from_millis(1));
                    winuser::mouse_event(winuser::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                    thread::sleep(Duration::from_millis(1));
                    winuser::mouse_event(winuser::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                    thread::sleep(Duration::from_millis(1));
                    winuser::SetCursorPos(initial_canvas_pos.0 + x, initial_canvas_pos.1 + y);
                    thread::sleep(Duration::from_millis(1));
                    winuser::mouse_event(winuser::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                }
            } else {
                unsafe {
                    if last_y != initial_canvas_pos.1 + y {
                        winuser::mouse_event(winuser::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                    }
                    winuser::SetCursorPos(initial_canvas_pos.0 + x, initial_canvas_pos.1 + y);
                    if last_y != initial_canvas_pos.1 + y {
                        thread::sleep(Duration::from_millis(1));
                        winuser::mouse_event(winuser::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                        last_y = initial_canvas_pos.1 + y;
                    }
                }
            }
            i = i + 1;
        }
    }

    unsafe {
        winuser::mouse_event(winuser::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
    }
}