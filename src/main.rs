#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::default::Default;
use std::fs;
use std::path::PathBuf;
use eframe::egui;
use egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded;
use egui_modal::Modal;
use json_editor::json::to_object;
use json_editor::json::value::JsonValueType;



#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
struct JsonEditor {
    current_file: Option<String>,
    #[serde(skip)]
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

fn main() -> eframe::Result {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "HAS Json editor",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(JsonEditor::new(cc)))
        }),
    )
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
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut app : JsonEditor = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            if let Some(name) = &app.current_file {
                let path = PathBuf::from(name);
                app.current_data = load_json(&path);
                if ! app.current_data.is_some() {
                    app.current_file = None;
                }

            }
            app
        } else {
            Default::default()
        }

    }

    fn show_menu(&mut self, ui: &mut egui::Ui, modal: &Modal) {
        use egui::menu;
        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
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

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}