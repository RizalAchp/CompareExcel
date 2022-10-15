#![allow(unused)]

use crate::{
    dpdcmpexcel::{CmpRslt, Comparison, LimitedVec},
    gui::mainwindow::thick_row,
};
use eframe::egui::{collapsing_header::HeaderResponse, *};

use super::{DisplayGui, View};

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
        let sz = ui.available_size();
        ui.push_id("source_table", |ui| {
            ui.set_height(sz.y * 0.5);
            ui.vertical(|ui| {
                egui_extras::TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(Layout::left_to_right(Align::Center))
                    .columns(egui_extras::Size::remainder().at_least(10.0), 4)
                    .resizable(false)
                    .header(20.0, table_header)
                    .body(|mut body| table_body(&mut body, &self.src, Color32::GREEN))
            });
        });
        ui.separator();
        ui.push_id("target_table", |ui| {
            ui.set_height(sz.y * 0.5);
            ui.vertical(|ui| {
                egui_extras::TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(Layout::left_to_right(Align::Center))
                    .columns(egui_extras::Size::remainder().at_least(10.0), 4)
                    .resizable(true)
                    .header(20.0, table_header)
                    .body(|mut body| table_body(&mut body, &self.tgt, Color32::BLUE))
            });
        });
    }
}

const HEADING: [&str; 4] = ["Nomor", "File", "Sheet", "Data"];

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
                ui.colored_label(
                    color,
                    item.oldindex
                        .unwrap_or(item.newindex.unwrap_or(0))
                        .to_string()
                );
            });
            row.col(|ui| {
                ui.colored_label(color, item.file.display_gui_text());
            });
            row.col(|ui| {
                ui.colored_label(color, &item.sheet);
            });
            row.col(|ui| {
                ui.style_mut().wrap = Some(false);
                ui.label(item.data.display_gui_text());
            });
        });
    }
}
