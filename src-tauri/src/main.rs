#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
extern crate serde;
use serde::{Deserialize, Serialize};
extern crate nes_rust;
use nes_rust::button;
use nes_rust::default_audio::DefaultAudio;
use nes_rust::default_display::DefaultDisplay;
use nes_rust::default_input::DefaultInput;
use nes_rust::rom::Rom;
use nes_rust::Nes;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[macro_use]
extern crate lazy_static;

#[tauri::command]
fn fib_rs(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fib_rs(n - 1) + fib_rs(n - 2)
    }
}
#[tauri::command]
fn sum_rs(v: Vec<u32>) -> u32 {
    let mut n = 0;
    for i in v {
        n += i;
    }
    n
}

#[tauri::command]
fn update_buffer(n: usize, val: u8) -> Vec<u8> {
    let v = vec![val; n];
    v
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum Button {
    Poweroff,
    Reset,
    Select,
    Start,
    Joypad1A,
    Joypad1B,
    Joypad1Up,
    Joypad1Down,
    Joypad1Left,
    Joypad1Right,
    Joypad2A,
    Joypad2B,
    Joypad2Up,
    Joypad2Down,
    Joypad2Left,
    Joypad2Right,
}

fn to_button_internal(button: Button) -> button::Button {
    match button {
        Button::Poweroff => button::Button::Poweroff,
        Button::Reset => button::Button::Reset,
        Button::Select => button::Button::Select,
        Button::Start => button::Button::Start,
        Button::Joypad1A => button::Button::Joypad1A,
        Button::Joypad1B => button::Button::Joypad1B,
        Button::Joypad1Up => button::Button::Joypad1Up,
        Button::Joypad1Down => button::Button::Joypad1Down,
        Button::Joypad1Left => button::Button::Joypad1Left,
        Button::Joypad1Right => button::Button::Joypad1Right,
        Button::Joypad2A => button::Button::Joypad2A,
        Button::Joypad2B => button::Button::Joypad2B,
        Button::Joypad2Up => button::Button::Joypad2Up,
        Button::Joypad2Down => button::Button::Joypad2Down,
        Button::Joypad2Left => button::Button::Joypad2Left,
        Button::Joypad2Right => button::Button::Joypad2Right,
    }
}
pub struct WasmNes {
    nes: Nes,
}
impl WasmNes {
    /// Creates a `WasmNes`
    pub fn new() -> Self {
        let input = Box::new(DefaultInput::new());
        let display = Box::new(DefaultDisplay::new());
        let audio = Box::new(DefaultAudio::new());
        let nes = Nes::new(input, display, audio);
        WasmNes { nes: nes }
    }

    /// Sets up NES rom
    ///
    /// # Arguments
    /// * `rom` Rom image binary `Uint8Array`
    pub fn set_rom(&mut self, contents: Vec<u8>) {
        self.nes.set_rom(Rom::new(contents));
    }

    /// Boots up
    pub fn bootup(&mut self) {
        self.nes.bootup();
    }

    /// Resets
    pub fn reset(&mut self) {
        self.nes.reset();
    }

    /// Executes a CPU cycle
    pub fn step(&mut self) {
        self.nes.step();
    }

    /// Executes a PPU (screen refresh) frame
    pub fn step_frame(&mut self) {
        self.nes.step_frame();
    }

    /// Copies RGB pixels of screen to passed RGBA pixels.
    /// The RGBA pixels length should be
    /// 245760 = 256(width) * 240(height) * 4(RGBA).
    /// A channel will be filled with 255(opaque).
    ///
    /// # Arguments
    /// * `pixels` RGBA pixels `Uint8Array` or `Uint8ClampedArray`
    pub fn update_pixels(&mut self, pixels: &mut [u8]) {
        self.nes.copy_pixels(pixels);
    }

    /// Copies audio buffer to passed `Float32Array` buffer.
    /// The length should be 4096.
    ///
    /// # Arguments
    /// * `buffer` Audio buffer `Float32Array`
    pub fn update_sample_buffer(&mut self, buffer: &mut [f32]) {
        self.nes.copy_sample_buffer(buffer);
    }

    /// Presses a pad button
    ///
    /// # Arguments
    /// * `button`
    pub fn press_button(&mut self, button: Button) {
        self.nes.press_button(to_button_internal(button));
    }

    /// Releases a pad button
    ///
    /// # Arguments
    /// * `buffer`
    pub fn release_button(&mut self, button: Button) {
        self.nes.release_button(to_button_internal(button));
    }
}
#[tauri::command]
fn to_button_internal_js(n: u32) {
    let btn = Button::try_from(n).unwrap();
    println!("Message from Rust: {:?}", btn);
}
// static mut input: Box<DefaultInput> = Box::new(DefaultInput::new());
// static mut display: Box<DefaultDisplay> = Box::new(DefaultDisplay::new());
// static mut audio: Box<DefaultAudio> = Box::new(DefaultAudio::new());

// static mut nes: Nes = Nes::new(input, display, audio);
// static mut global_nes: WasmNes = WasmNes { nes };

// lazy_static! {
//   let input = Box::new(DefaultInput::new());
//   let display = Box::new(DefaultDisplay::new());
//   let audio = Box::new(DefaultAudio::new());
//   let nes = Nes::new(input, display, audio);
//   static ref global_nes: WasmNes = WasmNes { nes: nes };
// }
use std::rc::Rc;
// static mut global_nes: WasmNes = WasmNes { nes: None };

// lazy_static! {
//     static ref global_nes: WasmNes = {
//         let input = Box::new(DefaultInput::new());
//         let display = Box::new(DefaultDisplay::new());
//         let audio = Box::new(DefaultAudio::new());
//         let nes = Nes::new(input, display, audio);
//         WasmNes { nes: nes }
//     };
// }

use once_cell::unsync::Lazy;
use once_cell::unsync::OnceCell;
static mut global_nes: OnceCell<&mut WasmNes> = OnceCell::new();

// static global_nes: Lazy<Mutex<WasmNes>> = Lazy::new(|| {
//     let mut m = HashMap::new();
//     m.insert(13, "Spica".to_string());
//     m.insert(74, "Hoyten".to_string());
//     Mutex::new(m)
// });

#[tauri::command]
fn create_nes() {
    // let c = Box::new(WasmNes { nes });        let input = Box::new(DefaultInput::new());
    // let input = Box::new(DefaultInput::new());
    // let display = Box::new(DefaultDisplay::new());
    // let audio = Box::new(DefaultAudio::new());
    // let nes = Box::new(Nes::new(input, display, audio));
    // let nes = Box::new(Nes::new(input, display, audio));
    // let b = Box::new(WasmNes { nes });

    // let wn = WasmNes { nes: Some(nes) };
    unsafe {
        global_nes.get_or_init(|| {
            let input = Box::new(DefaultInput::new());
            let display = Box::new(DefaultDisplay::new());
            let audio = Box::new(DefaultAudio::new());
            let nes = Nes::new(input, display, audio);
            let b = Box::new(WasmNes { nes: nes });
            let c = Box::leak(b);
            c
        });

        // global_nes = wn;
        // let c = &mut WasmNes { nes };
        // let c = Rc::new(Box::leak(nes));
        // let c: Option<WasmNes> = Some(wn);
        // std::mem::forget(c);
        // let bx = Box::leak(c);
        // global_nes = &c;
        // global_nes = &Some(c);
        // 将`c`从内存中泄漏，变成`'static`生命周期
        // global_nes = Some();
        // println!("{:?}", global_nes);
    }
}

#[tauri::command]
fn set_rom(rom: Vec<u8>) {
    // println!("set_rom {}", rom.len());
    unsafe {
        let g = global_nes.get_mut().unwrap();
        g.set_rom(rom);
    }
}

#[tauri::command]
fn bootup() {
    // println!("bootup  ");
    unsafe {
        let g = global_nes.get_mut().unwrap();
        g.bootup();
    }
}
#[tauri::command]
fn reset() {
    // println!("reset  ");
    unsafe {
        let g = global_nes.get_mut().unwrap();
        g.reset();
    }
}
#[tauri::command]
fn step() {
    // println!("step  ");
    unsafe {
        let g = global_nes.get_mut().unwrap();
        g.step();
    }
}

#[tauri::command]
fn step_frame() {
    // println!("step_frame  ");
    unsafe {
        let g = global_nes.get_mut().unwrap();
        g.step_frame();
    }
}
static mut global_pixels: &mut [u8] = &mut [0u8; 256 * 240 * 4];
#[tauri::command]
fn update_pixels() -> Vec<u8> {
    // println!("update_pixels");
    unsafe {
        let g = global_nes.get_mut().unwrap();
        g.update_pixels(global_pixels);
        global_pixels.iter().cloned().collect()
    }
}

#[tauri::command]
fn get_data() -> Vec<u8> {
    let v = [2u8; 256 * 240 * 4];
    v.iter().cloned().collect()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            fib_rs,
            sum_rs,
            update_buffer,
            to_button_internal_js,
            create_nes,
            set_rom,
            bootup,
            reset,
            step,
            step_frame,
            update_pixels,
            get_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
