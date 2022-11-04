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
    eframe::run_native("Mait", options, Box::new(|_cc| Box::new(MyApp::default())));
}

struct MyApp {
    command: String,
    timing_minute: String,
    timing_hour: String,
    timing_day_of_month: String,
    timing_month: String,
    timing_day_of_week: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            command: "echo 'hello'".to_owned(),
            timing_minute: "*".to_owned(),
            timing_hour: "*".to_owned(),
            timing_day_of_month: "*".to_owned(),
            timing_month: "*".to_owned(),
            timing_day_of_week: "*".to_owned(),
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
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Minute");
                    ui.text_edit_singleline(&mut self.timing_minute);
                });
                ui.horizontal(|ui| {
                    ui.label("Hour");
                    ui.text_edit_singleline(&mut self.timing_hour);
                });
                ui.horizontal(|ui| {
                    ui.label("Day Of Month");
                    ui.text_edit_singleline(&mut self.timing_day_of_month);
                });
                ui.horizontal(|ui| {
                    ui.label("Month");
                    ui.text_edit_singleline(&mut self.timing_month);
                });
                ui.horizontal(|ui| {
                    ui.label("Day Of Week");
                    ui.text_edit_singleline(&mut self.timing_day_of_week);
                });
            });
            if ui.button("Submit").clicked() {
                println!("cron has been submitted");
                fs::write(
                    env::var("USER").unwrap().to_string(),
                    format!(
                        "{timing_minute} {timing_hour} {timing_day_of_month} {timing_month} {timing_day_of_week} {command}\n",
                        timing_minute = self.timing_minute.to_owned(),
                        timing_hour = self.timing_hour.to_owned(),
                        timing_day_of_month = self.timing_day_of_month.to_owned(),
                        timing_month = self.timing_month.to_owned(),
                        timing_day_of_week = self.timing_day_of_week.to_owned(),
                        command = self.command.to_owned()
                    ),
                )
                .expect("couldn't write file");
            }
        });
    }
}
