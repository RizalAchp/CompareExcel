use std::{ops::Div, path::PathBuf};

use eframe::egui::*;

use crate::{dpdcmpexcel::compares::*, exec_async, gui::mainwindow::thick_row};

use super::{DisplayGui, Message, UnWrapGui, View};

#[derive(Debug)]
pub(super) struct InputTabel {
    pub(super) data: CmpData,
    pub(super) idx_sheet: usize,
    pub(super) idx_sort: usize,
    pub(super) idx_filter: usize,
    message_channel: (
        std::sync::mpsc::Sender<Message>,
        std::sync::mpsc::Receiver<Message>,
    ),
}
impl Default for InputTabel {
    fn default() -> Self {
        Self {
            data: Default::default(),
            idx_sheet: Default::default(),
            idx_sort: Default::default(),
            idx_filter: Default::default(),
            message_channel: std::sync::mpsc::channel(),
        }
    }
}

impl InputTabel {
    pub fn open_path(&mut self) {
        let future = rfd::AsyncFileDialog::new()
            .add_filter("ExcelFile", &["xlsx", "xlsb", "xlsm", "xls"])
            .set_title("Pilih Excel File Yang Akan Dibuka")
            .set_directory(super::HOME.unwrap_or_default())
            .pick_file();

        let message_sender = self.message_channel.0.clone();
        exec_async!({
            if let Some(file) = future.await {
                message_sender
                    .send(Message::FileOpen(CmpData::new(file.path())))
                    .ok();
            }
        });
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
            let sheet_selected = match self.data.sheets.get(self.idx_sheet) {
                Some(data) => data.to_owned(),
                None => String::new(),
            };
            self.data.set_selected_data(&sheet_selected).unwrap_gui();
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data.close();
        self.idx_sheet = 0;
    }

    #[inline]
    pub fn is_opened(&self) -> bool {
        !self.data.exl.is_none() && !self.data.sheets.is_empty()
    }

    pub fn draw_table(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            use egui_extras::{Size, TableBuilder};

            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(Layout::left_to_right(Align::Center))
                .columns(Size::remainder().at_least(10.0), self.data.size.w)
                .resizable(true)
                .header(20.0, |mut header| {
                    if !self.data.selected_data.is_empty() && self.data.has_header {
                        let iter = self.data.selected_data[0].clone().into_iter();
                        for (idx, item) in iter.enumerate() {
                            if header
                                .col(|ui| {
                                    ui.heading(item);
                                })
                                .interact(Sense::click().union(Sense::hover()))
                                .on_hover_text(format!(
                                    "Click to sort the table, current sort row: {}",
                                    idx
                                ))
                                .clicked()
                            {
                                self.data.sort(Some(idx)).unwrap_gui()
                            }
                        }
                    } else {
                        header.col(|ui| {
                            ui.heading("No Header");
                        });
                    }
                })
                .body(|mut body| {
                    if !self.data.selected_data.is_empty() {
                        let iters = if self.data.has_header {
                            self.data.selected_data[1..].iter().enumerate()
                        } else {
                            self.data.selected_data.iter().enumerate()
                        };
                        for (idx, item) in iters {
                            let row_height = if thick_row(idx) { 30.0 } else { 18.0 };
                            body.row(row_height, |mut row| {
                                for it in item {
                                    row.col(|ui| {
                                        ui.wrap_text();
                                        ui.small(it);
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
        {
            match self.message_channel.1.try_recv() {
                Ok(message) => match message {
                    Message::FileOpen(file) => self.set_data(file.unwrap_gui()),
                    _ => (),
                },
                Err(_) => {
                    ();
                }
            }
        }
        if !self.is_opened() {
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
                            eframe::egui::RichText::new(
                                "📂\nDROP OR CLICK\nTO OPEN FILE\nOR PASTE FROM WEB",
                            )
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
            if ui.button("Tutup file").clicked() {
                self.clear();
            } else {
                ui.horizontal(|ui| {
                    ui.separator();
                    ui.colored_label(Color32::BLUE, self.data.file.display_gui_text())
                        .on_hover_text(&self.data.file);
                    ui.separator();
                    let sheets: &Vec<String> = self.data.sheets.as_ref();
                    let cmb_changed = ComboBox::from_label("sheetexcel")
                        .show_index(ui, &mut self.idx_sheet, sheets.len(), |i| {
                            sheets[i].to_owned()
                        })
                        .changed();
                    if cmb_changed {
                        self.refresh();
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.data.has_header, "Has Header")
                        .on_hover_text("Check if excel has Header");
                    ui.separator();
                    if ui
                        .button("Sort")
                        .on_hover_text("Sort by max row available")
                        .clicked()
                    {
                        self.data.sort(None).unwrap_gui();
                    }
                    if ComboBox::from_label("SortBy Row")
                        .show_index(ui, &mut self.idx_sort, self.data.size.w, |i| i.to_string())
                        .on_hover_text("sort table by row, or you can click the header table")
                        .changed()
                    {
                        self.data.sort(Some(self.idx_sort)).unwrap_gui();
                    }
                    ui.separator();
                    if ui
                        .button("Filter")
                        .on_hover_text("filter by max row available")
                        .clicked()
                    {
                        self.data.filter(None).unwrap_gui();
                    }
                    if ComboBox::from_label("FilterBy Row")
                        .show_index(ui, &mut self.idx_filter, self.data.size.w, |i| {
                            i.to_string()
                        })
                        .on_hover_text("filter table by max row selected")
                        .changed()
                    {
                        self.data.filter(Some(self.idx_sort)).unwrap_gui();
                    }
                    ui.separator();
                    if ui.button("Undo Change").clicked() {
                        self.refresh();
                    }
                });
                ui.separator();
                self.draw_table(ui);
            }
        }
    }
}
