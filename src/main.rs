#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
extern crate device_query;
use mouse_rs::{types::keys::Keys, Mouse};
use eframe::egui;
use std::{thread, time};
use device_query::{DeviceQuery, DeviceState, Keycode};


fn main() -> Result<(), eframe::Error> {
    let mut click_mouse = false;
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    
    // Our application state:
    eframe::run_simple_native("autoclicker", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("dont use that in video games please");
            if ui.button("start").clicked() && click_mouse == true {
                move_mouse();
            }
        });
        input_checker();
    })
}

fn move_mouse() {
    let ten_millis = time::Duration::from_millis(10);

    loop {
        let mouse = Mouse::new();
        mouse.release(&Keys::LEFT).expect("Unable to release button");
        mouse.press(&Keys::LEFT).expect("Unable to press button");
        thread::sleep(ten_millis);

    }
}

pub fn input_checker() {
    let input_manager = device_query::DeviceState::new();
        let keys = input_manager.get_keys();
        if keys.contains(&Keycode::Q) {
                println!("The 'Q' key is pressed!");
                // Add a delay to avoid high CPU usage in the loop.
                std::thread::sleep(std::time::Duration::from_millis(100));
    

    }

}
