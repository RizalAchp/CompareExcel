#![allow(unused)]

use crate::{
    dpdcmpexcel::{deserializer::convert_csv_to_excel, CmpRslt, Comparison, LimitedVec},
    gui::mainwindow::thick_row,
};
use eframe::egui::{collapsing_header::HeaderResponse, *};

use super::{DisplayGui, UnWrapGui, View};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Targets {
    #[default]
    Source,
    Target,
}
#[derive(Debug)]
pub(super) struct OutputTable {
    src: Vec<CmpRslt>,
    tgt: Vec<CmpRslt>,
    show_table: Targets,
    message_channel: (
        std::sync::mpsc::Sender<super::Message>,
        std::sync::mpsc::Receiver<super::Message>,
    ),
}

impl Default for OutputTable {
    fn default() -> Self {
        Self {
            src: Default::default(),
            tgt: Default::default(),
            show_table: Default::default(),
            message_channel: std::sync::mpsc::channel(),
        }
    }
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

    pub fn on_sidebar(&mut self, ui: &mut Ui) {
        ui.small("Click `COMPARE INPUT` untuk membedakan antara 2 input excel yang sudah di dibuka, and otomatis window table akan berpaling ke tabel output. dimana output tabel hasil perbadaan didapatkan");
        if !self.src.is_empty() && !self.tgt.is_empty() {
            ui.wrap_text();
            ui.separator();
            if ui.button("Save Output").clicked() {
                self.save(Targets::Source);
                self.save(Targets::Target);
            };
            ui.separator();
        }
    }

    pub fn save(&mut self, idx: Targets) {
        let src: Vec<Vec<String>> = self
            .src
            .iter()
            .map(|(s)| {
                let mut data = vec![];
                data.extend([s.index.to_string(), s.file.to_owned(), s.sheet.to_owned()]);
                data.extend(s.data.to_owned());
                data
            })
            .collect();
        let tgt: Vec<Vec<String>> = self
            .src
            .iter()
            .map(|t| {
                let mut data = vec![];
                data.extend([t.index.to_string(), t.file.to_owned(), t.sheet.to_owned()]);
                data.extend(t.data.to_owned());
                data
            })
            .collect();

        let data = [tgt.as_slice(), src.as_slice()].concat();
        let fname = "OUTPUT_DIFF.xlsx".to_owned();
        let sheet = self.src[0].sheet.to_owned();

        let future = rfd::AsyncFileDialog::new()
            .add_filter("ExcelFile", &["xlsx", "xlsb", "xlsm", "xls"])
            .set_file_name(fname.as_str())
            .set_title("Save output to Excel File")
            .set_directory(super::HOME.unwrap_or_default())
            .save_file();
        let message_sender = self.message_channel.0.clone();
        crate::exec_async!({
            if let Some(file) = future.await {
                message_sender
                    .send(super::Message::IgnoredResult(
                        convert_csv_to_excel(data, file.path(), sheet).ok(),
                    ))
                    .ok();
            }
        });
    }
}

impl View for OutputTable {
    fn ui(&mut self, ui: &mut Ui) {
        use Targets::{Source, Target};
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.show_table, Source, "SHOW SUMBER");
            ui.radio_value(&mut self.show_table, Target, "SHOW TARGET");
        });
        ui.separator();
        ui.push_id("table_output_show", |ui| {
            ui.vertical(|ui| match self.show_table {
                Targets::Source => table_body(ui, &self.src, Color32::GREEN),
                Targets::Target => table_body(ui, &self.tgt, Color32::BLUE),
            })
        });
    }
}

const HEADING: [&str; 5] = ["Tag", "Nomor", "File", "Sheet", "Data"];

#[inline]
fn table_header(mut row: egui_extras::TableRow) {
    for head in HEADING {
        row.col(|ui| {
            ui.heading(head);
        });
    }
}

#[inline]
fn table_body(ui: &mut Ui, items: &Vec<CmpRslt>, color: Color32) {
    egui_extras::TableBuilder::new(ui)
        .striped(true)
        .cell_layout(Layout::left_to_right(Align::Center))
        .columns(egui_extras::Size::remainder().at_least(10.0), HEADING.len())
        .resizable(true)
        .header(20.0, table_header)
        .body(|mut body| {
            for (idx, item) in items.iter().enumerate() {
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.colored_label(color, item.tag.display_gui_text());
                    });
                    row.col(|ui| {
                        ui.colored_label(color, item.index.to_string());
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
        });
}
