#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui::*;
use rfd::FileDialog;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::env::consts::OS;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();

    eframe::run_native("Zeus Switch Game Merge Utility", options, Box::new(|_| Ok(Box::<MyApp>::default())))
}

#[derive(Default)]
struct MyApp {
    selected_version: Version,
    selected_files: Vec<String>, // Store selected file paths
    text_input: String, // Store text input
    command_done: Arc<Mutex<bool>>, // Track command execution status
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Version {
    NSP,
    XCI,
}

impl Default for Version {
    fn default() -> Self {
        Version::NSP
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark()); // Set dark theme
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Zeus Switch Game Merge Utility").font(FontId::proportional(25.0))
                    .underline());
            });

            ui.add_space(5.0);

            ui.label(RichText::new("Please select the version of the import files").color(Color32::RED));

            ui.add_space(5.0);
            ui.group(|ui| {
                ui.label("Select Version:");
                ui.radio_value(&mut self.selected_version, Version::NSP, "NSP");
                ui.radio_value(&mut self.selected_version, Version::XCI, "XCI");
            });

            ui.add_space(10.0);

            // Button to open file dialog
            if ui.button("Select Game Files").clicked() {
                if let Some(files) = FileDialog::new().pick_files() {
                    self.selected_files = files.iter()
                        .filter_map(|file| file.to_str().map(|s| s.to_string())) // Convert paths to strings
                        .collect();
                }
            }

            ui.add_space(5.0);
            ui.separator();

            // Add a text entry field at the bottom
            ui.add_space(5.0);
            ui.label("Enter the name of the output game name:");
            ui.text_edit_singleline(&mut self.text_input);

            ui.add_space(5.0);
            // Button to run the command
            if ui.button("Start").clicked() {
                if self.text_input.is_empty() {
                    ui.label("Please enter a filename."); // Notify user to enter a filename
                    return;
                }

                let extension = match self.selected_version {
                    Version::NSP => "nsp",
                    Version::XCI => "xci",
                };

                let output_file = format!("{}.{}", self.text_input, extension);
                let escaped_files: Vec<String> = self.selected_files.iter()
                    .map(|file| format!("\"{}\"", file))
                    .collect();
                let escaped_output_file = format!("\"{}\"", output_file);

                // Track command completion status
                let command_done = Arc::new(Mutex::new(false));
                let command_done_clone = Arc::clone(&command_done);

                // Run the command in a separate thread
                let files_clone = escaped_files.clone();
                let output_file_clone = escaped_output_file.clone();

                thread::spawn(move || {
                    let command_str = if OS == "windows" {
                        // For Windows
                        format!(
                            "copy /b {} {}",
                            files_clone.join(" + "),
                            output_file_clone
                        )
                    } else {
                        // For macOS and other Unix-like systems
                        format!(
                            "cat {} > {}",
                            files_clone.join(" "),
                            output_file_clone
                        )
                    };

                    // Execute the command
                    let output = if OS == "windows" {
                        Command::new("cmd").args(&["/C", &command_str]).output()
                    } else {
                        Command::new("sh").arg("-c").arg(&command_str).output()
                    };

                    match output {
                        Ok(output) => {
                            if !output.status.success() {
                                eprintln!("Command failed with output: {:?}", String::from_utf8_lossy(&output.stderr));
                            } else {
                                println!("Command executed successfully: {:?}", String::from_utf8_lossy(&output.stdout));
                            }
                        }
                        Err(err) => {
                            eprintln!("Failed to execute command: {}", err);
                        }
                    }

                    // Set command done status
                    let mut done = command_done_clone.lock().unwrap();
                    *done = true;
                });

                self.command_done = command_done; // Update the command status in the main app
            }

            // Display "Done" label if the command is finished
            if *self.command_done.lock().unwrap() {
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);
                ui.label("Finished!");
            }
        });
    }
}

