#![allow(unused)]

use crate::{
    dpdcmpexcel::{CmpRslt, LimitedVec},
    gui::mainwindow::thick_row,
};
use eframe::egui::*;

use super::View;

pub(super) struct OutputTable {
    pub(super) data: Option<Vec<CmpRslt>>,
}

impl Default for OutputTable {
    fn default() -> Self {
        Self { data: None }
    }
}
impl OutputTable {
    pub fn muting(&mut self, d: &Vec<CmpRslt>) {
        self.data = Some(d.to_vec())
    }
    pub fn clear(&mut self) {
        self.data = None;
    }
}

impl View for OutputTable {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.vertical(|ui| {
            use egui_extras::{Size, TableBuilder};

            let mut table = TableBuilder::new(ui)
                .striped(true)
                .cell_layout(eframe::egui::Layout::left_to_right(
                    eframe::egui::Align::Center,
                ))
                .column(Size::initial(60.0).at_least(40.0))
                .column(Size::initial(60.0).at_least(40.0))
                .column(Size::initial(60.0).at_least(40.0))
                .column(Size::initial(60.0).at_least(40.0))
                .column(Size::remainder().at_least(60.0))
                .resizable(false);

            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Nomor");
                    });
                    header.col(|ui| {
                        ui.heading("Sign");
                    });
                    header.col(|ui| {
                        ui.heading("Sheet");
                    });
                    header.col(|ui| {
                        ui.heading("Data");
                    });
                })
                .body(|mut body| {
                    if let Some(items) = &self.data {
                        for (idx, item) in items.iter().enumerate() {
                            let row_height = if thick_row(idx) { 30.0 } else { 18.0 };
                            let color = match item.sign.as_str() {
                                "+" => Color32::LIGHT_RED,
                                "-" => Color32::LIGHT_GREEN,
                                _ => Color32::GRAY,
                            };

                            body.row(row_height, |mut row| {
                                row.col(|ui| {
                                    ui.label(
                                        item.oldindex
                                            .unwrap_or(item.newindex.unwrap_or(0))
                                            .to_string(),
                                    );
                                });
                                row.col(|ui| {
                                    ui.colored_label(color, &item.sign);
                                });
                                row.col(|ui| {
                                    ui.colored_label(color, &item.sheet);
                                });
                                row.col(|ui| {
                                    ui.style_mut().wrap = Some(false);
                                    ui.label(format!(
                                        "{}",
                                        LimitedVec {
                                            0: item.data.to_owned()
                                        }
                                    ));
                                });
                            });
                        }
                    }
                })
        });
    }
}
