use std::{fmt::Display, ops::Div, path::PathBuf};

use eframe::egui::*;

use crate::{
    dpdcmpexcel::{compares::*, deserializer::validate, SortVec},
    gui::mainwindow::thick_row,
};

use super::{UnWrapGui, View};

pub(super) struct InputTabel {
    pub(super) data: Option<CmpData>,
    pub(super) selected_idx: usize,
    pub(super) selected_data: Vec<Vec<String>>,
    pub(crate) size: (usize, usize),
}
impl InputTabel {
    pub fn open_path(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            self.set_data(CmpData::new(&path).unwrap_gui());
        }
    }
    pub fn set_data(&mut self, d: Option<CmpData>) {
        self.data = d;
        self.refresh();
    }
    pub fn refresh(&mut self) {
        if let Some(data) = &mut self.data {
            let sheet_selected = match data.sheets.get(self.selected_idx) {
                Some(data) => data.to_owned(),
                None => String::new(),
            };
            match data.get_deserialized_data(&sheet_selected).unwrap_gui() {
                Some(k) => {
                    self.selected_data.clear();
                    self.selected_data = k.0;
                    self.size = k.1;
                }
                None => (),
            }
        };
    }

    pub fn sort_table(&mut self, idx: usize) {
        self.selected_data
            .sort_by_col(idx, self.size.1)
            .unwrap_gui();
    }

    pub fn clear(&mut self) {
        if let Some(data) = &self.data {
            drop(data);
        }
        self.selected_data.clear();
        self.data = None;
    }

    fn hovered_inactive(&self) -> bool {
        if self.data.is_none() || self.data.as_ref().unwrap().sheets.is_empty() {
            true
        } else {
            false
        }
    }
}
impl Default for InputTabel {
    fn default() -> Self {
        Self {
            data: None,
            selected_idx: 0,
            selected_data: Vec::new(),
            size: (0, 0),
        }
    }
}

impl View for InputTabel {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        if self.hovered_inactive() {
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
                    return self.clear();
                }
                ui.label(format!("{}", self.data.as_ref().unwrap().file));
                ui.separator();
                let sheets: &Vec<String> = self.data.as_ref().unwrap().sheets.as_ref();
                let tmp_idx = self.selected_idx.clone();
                ComboBox::from_label("sheetexcel").show_index(
                    ui,
                    &mut self.selected_idx,
                    sheets.len(),
                    |i| sheets[i].to_owned(),
                );
                if tmp_idx != self.selected_idx {
                    self.refresh();
                }
            });
            ui.separator();
            ui.vertical(|ui| {
                use egui_extras::{Size, TableBuilder};

                TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(eframe::egui::Layout::left_to_right(
                        eframe::egui::Align::Center,
                    ))
                    .columns(Size::remainder().at_least(10.0), self.size.1)
                    .resizable(true)
                    .header(20.0, |mut header| {
                        if !self.selected_data.is_empty() {
                            let iter = self.selected_data[0].clone().into_iter();
                            for (idx, item) in iter.enumerate() {
                                if header
                                    .col(|ui| {
                                        ui.heading(item);
                                    })
                                    .interact(Sense::click())
                                    .clicked()
                                {
                                    self.sort_table(idx)
                                }
                            }
                        }
                    })
                    .body(|mut body| {
                        if !self.selected_data.is_empty() {
                            for (idx, item) in self.selected_data.iter().enumerate() {
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
