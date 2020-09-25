mod canvas;

use canvas::{color, mxcfb_rect, Canvas, FramebufferDraw, Point2, Vector2};
use fxhash::FxHashMap;
use libremarkable::input::{
    ev::EvDevContext, multitouch::Finger, multitouch::MultitouchEvent, InputDevice, InputEvent,
};
use std::{io::stdout, io::Write};

#[derive(PartialEq)]
struct Key {
    name: &'static str,
    bounds: mxcfb_rect,
    is_black: bool,
    is_pressed: bool,
}

impl Key {
    fn new(name: &'static str, x: u32, y: u32, w: u32, h: u32, is_black: bool) -> Self {
        Self {
            name,
            bounds: mxcfb_rect {
                left: 1404 - y - h,
                top: x,
                width: h,
                height: w,
            },
            is_black,
            is_pressed: false,
        }
    }
}

fn main() {
    let mut canvas = Canvas::new();

    let (input_tx, input_rx) = std::sync::mpsc::channel::<InputEvent>();
    //EvDevContext::new(InputDevice::GPIO, input_tx.clone()).start();
    EvDevContext::new(InputDevice::Multitouch, input_tx).start();
    //EvDevContext::new(InputDevice::Wacom, input_tx.clone()).start();

    const BASE_Y: u32 = 350;
    const W_W: u32 = 213;
    const W_H: u32 = 1054;
    const B_W: u32 = 115;
    const B_H: u32 = 631;
    const M_W: u32 = 200;
    const M_H: u32 = 150;

    // Map/Octave selection
    let m1 = Key::new("m1", 1872 - M_W * 7, 0, M_W, M_H, false);
    let m2 = Key::new("m2", 1872 - M_W * 6, 0, M_W, M_H, false);
    let m3 = Key::new("m3", 1872 - M_W * 5, 0, M_W, M_H, false);
    let m4 = Key::new("m4", 1872 - M_W * 4, 0, M_W, M_H, false);
    let m5 = Key::new("m5", 1872 - M_W * 3, 0, M_W, M_H, false);
    let m6 = Key::new("m6", 1872 - M_W * 2, 0, M_W, M_H, false);
    let m7 = Key::new("m7", 1872 - M_W * 1, 0, M_W, M_H, false);

    // White keys in order
    let w1 = Key::new("w1", W_W * 0, BASE_Y, W_W, W_H, false);
    let w2 = Key::new("w2", W_W * 1, BASE_Y, W_W, W_H, false);
    let w3 = Key::new("w3", W_W * 2, BASE_Y, W_W, W_H, false);
    let w4 = Key::new("w4", W_W * 3, BASE_Y, W_W, W_H, false);
    let w5 = Key::new("w5", W_W * 4, BASE_Y, W_W, W_H, false);
    let w6 = Key::new("w6", W_W * 5, BASE_Y, W_W, W_H, false);
    let w7 = Key::new("w7", W_W * 6, BASE_Y, W_W, W_H, false);
    let w8 = Key::new("w8", W_W * 7, BASE_Y, W_W, W_H, false); // Technicially the next w1. But seems to be used this way

    // Black keys in order
    let b1 = Key::new("b1", W_W * 1 - B_W / 2, BASE_Y, B_W, B_H, true);
    let b2 = Key::new("b2", W_W * 2 - B_W / 2, BASE_Y, B_W, B_H, true);
    let b3 = Key::new("b3", W_W * 4 - B_W / 2, BASE_Y, B_W, B_H, true);
    let b4 = Key::new("b4", W_W * 5 - B_W / 2, BASE_Y, B_W, B_H, true);
    let b5 = Key::new("b5", W_W * 6 - B_W / 2, BASE_Y, B_W, B_H, true);

    let mut keys = vec![
        m1, m2, m3, m4, m5, m6, m7, w1, w2, w3, w4, w5, w6, w7, w8, b1, b2, b3, b4, b5,
    ];

    canvas.clear();
    draw_keys(&mut canvas, &keys);
    canvas.update_full();

    let mut fingers: FxHashMap<i32, Finger> = Default::default();

    while let Ok(event) = input_rx.recv() {
        if let InputEvent::MultitouchEvent { event } = event {
            match event {
                MultitouchEvent::Press { finger } | MultitouchEvent::Move { finger } => {
                    fingers.insert(finger.tracking_id, finger);
                }
                MultitouchEvent::Release { finger } => {
                    fingers.remove(&finger.tracking_id);
                }
                MultitouchEvent::Unknown => {}
            }

            let pressed_key_names: Vec<&'static str> = fingers
                .values()
                .into_iter()
                .map(|f| key_at(f.pos, &keys))
                .filter(|key_option| key_option.is_some())
                .map(|key_option| key_option.unwrap().name)
                .collect();

            for key in keys.iter_mut() {
                let new_is_pressed = pressed_key_names.contains(&key.name);
                if !key.is_pressed && new_is_pressed {
                    println!("PRESS {}", key.name);
                    stdout().flush().unwrap();
                    key.is_pressed = new_is_pressed;
                } else if key.is_pressed && !new_is_pressed {
                    println!("RELEASE {}", key.name);
                    stdout().flush().unwrap();
                    key.is_pressed = new_is_pressed;
                }
            }
        }
    }
}

fn key_at<'a>(pos: Point2<u16>, keys: &'a [Key]) -> Option<&'a Key> {
    let mut found_key: Option<&'a Key> = None;
    for key in keys.iter() {
        if Canvas::is_hitting(pos, key.bounds) {
            if let Some(old_key) = found_key {
                if key.is_black && !old_key.is_black {
                    found_key = Some(key);
                }
            } else {
                found_key = Some(key);
            }
        }
    }

    found_key
}

fn draw_keys(canvas: &mut Canvas, keys: &[Key]) {
    for key in keys.iter() {
        if key.is_black {
            canvas.framebuffer_mut().fill_rect(
                Point2 {
                    x: key.bounds.left as i32,
                    y: key.bounds.top as i32,
                },
                Vector2 {
                    x: key.bounds.width,
                    y: key.bounds.height,
                },
                color::BLACK,
            );
        } else {
            canvas.framebuffer_mut().draw_rect(
                Point2 {
                    x: key.bounds.left as i32,
                    y: key.bounds.top as i32,
                },
                Vector2 {
                    x: key.bounds.width,
                    y: key.bounds.height,
                },
                4,
                color::BLACK,
            );
        }
    }
}
