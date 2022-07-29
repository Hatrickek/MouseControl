#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use crate::egui::Key;

use windows_sys::{
    Win32::Foundation::*,Win32::UI::WindowsAndMessaging::*,
};

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.resizable = false;
    options.default_theme = eframe::Theme::Dark;
    let size: egui::Vec2 = egui::Vec2 { x: 300f32, y: 300f32 };
    options.initial_window_size = Option::from(size);
    let winsize_x: i32;
    let winsize_y: i32;
    unsafe { winsize_x = GetSystemMetrics(SM_CXSCREEN); };
    unsafe { winsize_y = GetSystemMetrics(SM_CYSCREEN); };
    let control: MouseControl = MouseControl { locked: false, size_x: winsize_x, size_y: winsize_y };
    eframe::run_native(
        "MouseControl",
        options,
        Box::new(|_cc| Box::new(control)),
    );
}

struct MouseControl {
    locked: bool,
    size_x: i32,
    size_y: i32,
}

impl eframe::App for MouseControl {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mouse Control 🚀");
            ui.horizontal(|ui| {});
            if ui.button("Center cursor").clicked() {
                center_mouse(self);
            }
            if ui.button("Lock cursor").clicked() {
                self.locked = !self.locked;
            }
        });
        if ctx.input().key_pressed(Key::End) {
            self.locked = !self.locked;
        }
        if ctx.input().key_pressed(Key::Home) {
            center_mouse(self);
        }
        lock_mouse(self);
    }
}

fn lock_mouse(control: &mut MouseControl) {
    if control.locked {
        unsafe {
            SetCursorPos(0, 5000);
        }
    } else {
        control.locked = false;
    }
}

fn center_mouse(control: &mut MouseControl) {
    unsafe {
        SetCursorPos(control.size_x / 2, control.size_y / 2);
    }
}