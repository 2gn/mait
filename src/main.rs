#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use std::env;
use std::fs;
// use std::io;
// use std::io::prelude::*;
// use std::os::unix;
// use std::path::Path;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    command: String,
    timing: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            command: "echo 'hello'".to_owned(),
            timing: "0 0 0 * *".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("command");
                ui.text_edit_singleline(&mut self.command);
            });
            if ui.button("Submit").clicked() {
                println!("cron has been submitted");
                fs::write(
                    env::var("USER").unwrap().to_string(),
                    format!(
                        "{timing} {command}\n",
                        timing = self.timing.to_owned(),
                        command = self.command.to_owned()
                    ),
                )
                .expect("couldn't write file");
            }
        });
    }
}
