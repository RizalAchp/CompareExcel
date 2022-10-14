#![allow(unused)]

use crate::{
    dpdcmpexcel::{CmpRslt, LimitedVec, Comparison},
    gui::mainwindow::thick_row,
};
use eframe::egui::{collapsing_header::HeaderResponse, *};

use super::View;

#[derive(Debug, Default)]
pub(super) struct OutputTable {
    src: Vec<CmpRslt>,
    tgt: Vec<CmpRslt>,
}

impl OutputTable {
    #[inline]
    pub fn set_src(&mut self, src: Vec<CmpRslt>) {
        self.src = src.to_owned()
    }
    #[inline]
    pub fn set_tgt(&mut self, tgt: Vec<CmpRslt>) {
        self.tgt = tgt.to_owned()
    }
    #[inline]
    pub fn clear(&mut self) {
        self.tgt.clear();
        self.src.clear();
    }
}

impl View for OutputTable {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            egui_extras::TableBuilder::new(ui)
                .striped(true)
                .cell_layout(Layout::left_to_right(Align::Center))
                .columns(egui_extras::Size::remainder().at_least(10.0), 5)
                .resizable(false)
                .header(20.0, table_header)
                .body(|mut body| table_body(&mut body, &self.src, Color32::GREEN))
        });
        ui.separator();
        ui.vertical(|ui| {
            egui_extras::TableBuilder::new(ui)
                .striped(true)
                .cell_layout(Layout::left_to_right(Align::Center))
                .columns(egui_extras::Size::remainder().at_least(10.0), 5)
                .resizable(false)
                .header(20.0, table_header)
                .body(|mut body| table_body(&mut body, &self.tgt, Color32::BLUE))
        });
    }
}

const HEADING: [&str; 5] = ["Nomor", "Sign", "File", "Sheet", "Data"];

#[inline]
fn table_header(mut row: egui_extras::TableRow) {
    for head in HEADING {
        row.col(|ui| {
            ui.heading(head);
        });
    }
}

#[inline]
fn table_body(body: &mut egui_extras::TableBody, items: &Vec<CmpRslt>, color: Color32) {
    for (idx, item) in items.iter().enumerate() {
        body.row(30.0, |mut row| {
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
                ui.label(&item.file);
            });
            row.col(|ui| {
                ui.colored_label(color, &item.sheet);
            });
            row.col(|ui| {
                ui.style_mut().wrap = Some(false);
                ui.label(format!(
                    "{}",
                    LimitedVec {
                        0: item.data.to_owned(),
                    }
                ));
            });
        });
    }
}
