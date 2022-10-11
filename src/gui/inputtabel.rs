use std::{fmt::Display, ops::Div, path::PathBuf, thread};

use eframe::egui::*;

use crate::{
    dpdcmpexcel::{compares::*, deserializer::validate, SortVec},
    gui::mainwindow::thick_row,
};

use super::{UnWrapGui, View};

#[cfg(linux)]
const HOME: &'static str = env!("HOME");

#[cfg(windows)]
const HOME: &'static str = env!("USERPROFILE");

#[derive(Default, Debug)]
pub(super) struct InputTabel {
    pub(super) data: CmpData,
    pub(super) selected_idx: usize,
    pub(super) selected_data: Vec<Vec<String>>,
    pub(crate) size: (usize, usize),
    pub(crate) has_header: bool,
    pub(crate) is_filtered: bool,
}
impl InputTabel {
    pub fn open_path(&mut self) {
        if let Some(path) = thread::spawn(move || {
            rfd::FileDialog::new()
                .add_filter("ExcelFile", &["xlsx", "xlsb", "xlsm", "xls"])
                .set_title("Pilih Excel File Yang Akan Dibuka")
                .set_directory(HOME)
                .pick_file()
        })
        .join()
        .unwrap_or(None)
        {
            self.set_data(CmpData::new(&path).unwrap_gui());
        }
    }
    #[inline]
    pub fn set_data(&mut self, d: CmpData) {
        self.data = d;
        self.refresh();
    }

    #[inline]
    pub fn refresh(&mut self) {
        if self.data.exl.is_none() {
            ()
        } else {
            self.selected_data.clear();
            let sheet_selected = match self.data.sheets.get(self.selected_idx) {
                Some(data) => data.to_owned(),
                None => String::new(),
            };
            (self.selected_data, self.size) = self
                .data
                .get_deserialized_data(&sheet_selected)
                .unwrap_gui();
        }
    }

    #[inline]
    pub fn sort_table(&mut self, idx: usize) {
        self.selected_data.sort_by_col(idx).unwrap_gui();
    }

    #[inline]
    pub fn clear(&mut self) {
        drop(&mut self.data);
        self.selected_data.clear();
    }

    #[inline]
    pub fn hovered_inactive(&self) -> bool {
        self.data.exl.is_none() || self.data.sheets.is_empty()
    }

    #[inline]
    pub fn draw_table(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            use egui_extras::{Size, TableBuilder};

            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(Layout::left_to_right(Align::Center))
                .columns(Size::remainder().at_least(10.0), self.size.1)
                .resizable(true)
                .header(20.0, |mut header| {
                    if !self.selected_data.is_empty() && self.has_header {
                        let iter = self.selected_data[0].clone().into_iter();
                        for (idx, item) in iter.enumerate() {
                            let clicked = header
                                .col(|ui| {
                                    ui.heading(item);
                                })
                                .interact(Sense::click())
                                .clicked();
                            if clicked && self.is_filtered {
                                self.sort_table(idx)
                            }
                        }
                    } else {
                        header.col(|ui| {
                            ui.heading("No Header");
                        });
                    }
                })
                .body(|mut body| {
                    if !self.selected_data.is_empty() {
                        let iters = self.selected_data[1..].iter().enumerate();
                        for (idx, item) in iters {
                            let row_height = if thick_row(idx) { 30.0 } else { 18.0 };
                            body.row(row_height, |mut row| {
                                for it in item {
                                    row.col(|ui| {
                                        ui.label(it);
                                    });
                                }
                            });
                        }
                    }
                })
        });
    }
}

impl View for InputTabel {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        if self.hovered_inactive() || !ui.ctx().input().raw.hovered_files.is_empty() {
            if !ui.ctx().input().raw.dropped_files.is_empty() {
                if let Some(path) = ui.ctx().input().raw.dropped_files.clone().first() {
                    self.set_data(
                        CmpData::new(&path.path.as_ref().unwrap_or(&PathBuf::default()))
                            .unwrap_gui(),
                    );
                }
            }
            if !ui.ui_contains_pointer() {
                return;
            } else {
                let rect = ui.max_rect();
                if ui
                    .centered_and_justified(|ui| {
                        ui.painter().rect_filled(
                            rect,
                            0.0,
                            eframe::egui::Color32::from_black_alpha(192),
                        );
                        ui.label(
                            eframe::egui::RichText::new("📂\nDROP OR OPEN FILE")
                                .size(rect.height().div(10f32)),
                        )
                    })
                    .response
                    .interact(Sense::click())
                    .clicked()
                {
                    self.open_path();
                }
            }
        } else {
            ui.horizontal(|ui| {
                if ui.button("Tutup file").clicked() {
                    self.clear();
                }
                ui.separator();
                ui.colored_label(
                    Color32::BLUE,
                    RichText::new(format!("{}", &self.data.file)).monospace(),
                );
                ui.separator();
                let sheets: &Vec<String> = self.data.sheets.as_ref();
                let cmb_changed = ComboBox::from_label("sheetexcel")
                    .show_index(ui, &mut self.selected_idx, sheets.len(), |i| {
                        sheets[i].to_owned()
                    })
                    .changed();
                if cmb_changed {
                    self.refresh();
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.has_header, "Punya Header")
                    .on_hover_text("Check if excel has Header");
                ui.separator();
                if ui.button("filter row").clicked() {
                    self.selected_data.filter_col(self.size.1).unwrap_gui();
                }
            });
            ui.separator();
            self.draw_table(ui);
        }
    }
}

pub struct ThePath(pub Option<PathBuf>);
impl Display for ThePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(p) => match validate(p) {
                Ok(_) => write!(
                    f,
                    "\n📃 {}",
                    p.file_name().unwrap().to_str().unwrap_or("???")
                ),
                Err(_) => write!(f, "\n❎ invalid file {}", p.to_str().unwrap_or("???")),
            },
            None => write!(f, "\n???"),
        }
    }
}
