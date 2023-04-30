use std::ptr::null_mut;
use winapi::shared::windef::RECT;
use winapi::um::winuser;

pub struct PaintPosition {
    pub initial_palette_pos: (i32, i32),
    pub initial_canvas_pos: (i32, i32),
}

impl PaintPosition {
    pub fn new() -> PaintPosition {
        let window_name = "Untitled - Paint\0";
        let hwnd = unsafe { winuser::FindWindowA(null_mut(), window_name.as_ptr() as *const i8) };
        if hwnd != null_mut() {
            let mut rect: RECT = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            unsafe { winuser::GetWindowRect(hwnd, &mut rect) };
            PaintPosition {
                initial_palette_pos: (rect.left + 642, rect.top + 70),
                initial_canvas_pos: (rect.left + 32, rect.top + 170),
            }
        } else {
            panic!("Cannot find active Paint window.");
        }
    }
}
