mod imager;
mod mspaint;

use dotenv::dotenv;
use image::{self, GenericImageView};
use mspaint::PaintPosition;
use redis::{self, Commands};
use std::env;
use std::thread;
use std::time::Duration;
use winapi::um::winuser;

fn main() {
    dotenv().ok();

    let delay: u64 = 1;

    let paint_window: PaintPosition = PaintPosition::new();
    let initial_canvas_pos: &(i32, i32) = &paint_window.initial_canvas_pos;
    let initial_palette_pos: &(i32, i32) = &paint_window.initial_palette_pos;

    let img: image::DynamicImage = image::open("example.png").unwrap();
    let pixels: Vec<image::Rgb<u8>> = imager::get_pixels(&img);
    let dimension: (u32, u32) = img.dimensions();

    unsafe {
        winuser::SetCursorPos(initial_palette_pos.0, initial_palette_pos.1);
        winuser::mouse_event(winuser::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
        thread::sleep(Duration::from_millis(delay));
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

            let closest_color: (i32, i32) =
                imager::get_closest_color_from_palette(&pixels[i], &initial_palette_pos);

            if last_color.0 != closest_color.0 || last_color.1 != closest_color.1 {
                last_color = closest_color;
                unsafe {
                    winuser::mouse_event(winuser::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                    thread::sleep(Duration::from_millis(delay));
                    winuser::SetCursorPos(closest_color.0, closest_color.1);
                    thread::sleep(Duration::from_millis(delay));
                    for _ in 0..2 {
                        winuser::mouse_event(winuser::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                        thread::sleep(Duration::from_millis(delay));
                        winuser::mouse_event(winuser::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                        thread::sleep(Duration::from_millis(delay));
                    }
                    winuser::SetCursorPos(initial_canvas_pos.0 + x, initial_canvas_pos.1 + y);
                    thread::sleep(Duration::from_millis(delay));
                    winuser::mouse_event(winuser::MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                }
            } else {
                unsafe {
                    if last_y != initial_canvas_pos.1 + y {
                        winuser::mouse_event(winuser::MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                    }
                    winuser::SetCursorPos(initial_canvas_pos.0 + x, initial_canvas_pos.1 + y);
                    if last_y != initial_canvas_pos.1 + y {
                        thread::sleep(Duration::from_millis(delay));
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

    let redis_conn_str: &str =
        &env::var("REDIS_CONN").expect("Cannot connect to Redis because credential not set.");
    let redis_client: redis::Client =
        redis::Client::open(redis_conn_str).expect("Cannot connect to Redis.");
    let mut redis_conn: redis::Connection = redis_client.get_connection().unwrap();

    let _ = redis_conn.set_ex::<&str, bool, u8>("naila:draw:done", true, 60);
}
