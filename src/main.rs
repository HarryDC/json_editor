#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::fs;
use std::path::PathBuf;
use eframe::egui;
use egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded;
use egui::WidgetType::CollapsingHeader;
use egui_modal::Modal;
use json_editor::json::to_object;
use json_editor::json::value::JsonValueType;

fn main() -> eframe::Result {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<JsonEditor>::default())
        }),
    )
}

struct JsonEditor {
    current_file: Option<String>,
    current_data: Option<JsonValueType>
 }

impl Default for JsonEditor {
    fn default() -> Self {
        Self {
            current_file: None, // Restore path from stored path
            current_data: None,
        }
    }
}

fn load_json(path: &PathBuf) -> Option<JsonValueType> {
    let result = fs::read_to_string(path).expect("Could not Open File");
    match to_object(result.as_ref()) {
        Err(_) => None,
        Ok(result) => Some(result)
    }
}

fn simple_json_view(ui: &mut egui::Ui, value: &JsonValueType) {
    egui::ScrollArea::both().scroll_bar_visibility(VisibleWhenNeeded)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
            |ui| draw_json_value(ui, value));
        },
    );
}

fn draw_json_value(ui: &mut egui::Ui, value: &JsonValueType) {
    match value {
        JsonValueType::JsonTypeNull => {ui.label("null");}
        JsonValueType::JsonTypeBool(val) => {if *val {ui.label("true");}
        else {ui.label("false");}}
        JsonValueType::JsonTypeNumber(val) => {ui.label(val.to_string());}
        JsonValueType::JsonTypeObject(val) => {
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                for item in val {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.label(item.0);
                        draw_json_value(ui, &item.1);
                    });
                }
            });
        }
        JsonValueType::JsonTypeArray(val) => {
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                for (i, item) in val.iter().enumerate()
                {
                    egui::CollapsingHeader::new(i.to_string()).show(ui, |ui| draw_json_value(ui,item) );
                }
            }
            );
        }
        JsonValueType::JsonTypeString(val) => {ui.label(val);}
    }
}

impl JsonEditor {

    fn show_menu(&mut self, ui: &mut egui::Ui, modal: &Modal) {
        use egui::menu;
        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked()  {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.current_data = load_json(&path);
                        if self.current_data.is_some() {
                            self.current_file = Some(path.display().to_string());
                        } else {
                            // Check for parsing error vs file error
                            modal.dialog()
                                .with_title("Loading Failed")
                                .with_body("Loading Failed")
                                .open();
                        }
                    }
                }
            });
        });
    }

    fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
    }
}

impl eframe::App for JsonEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut modal = Modal::new(ctx, "dialog");

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.show_menu(ui, &modal);
        });

        match self.current_data.as_ref() {
            None => {},
            Some(value) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    simple_json_view(ui, value);
                });
            }
        }

        modal.show_dialog();
    }
}