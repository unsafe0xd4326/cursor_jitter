#![windows_subsystem = "windows"]
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^Hide cmd

// usings
use { 
    winapi::{
        um::winuser::{GetCursorPos, SetCursorPos},
        shared::windef::POINT
    },
    rand::prelude::*,
};


fn main() {
    let mut rng = rand::thread_rng();
    // init cursor struct
    let mut cursor = POINT {x: 0, y: 0};
    let mut x : i32 = 0;
    let mut y : i32 = 0;
    let force = 2;
    loop {
        unsafe {
            GetCursorPos(&mut cursor);
            x = &cursor.x + rng.gen::<i32>() % force;
            y = &cursor.y + rng.gen::<i32>() % force;
            // move cursor
            SetCursorPos(x, y);
        }
    }
}

