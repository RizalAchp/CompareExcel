use std::cell::RefCell;

use eframe::egui::style::Margin;
use eframe::egui::*;

use crate::dpdcmpexcel::compares::Comparison;
use crate::dpdcmpexcel::DpdError;

use super::inputtabel::InputTabel;
use super::outputtabel::OutputTable;
use super::{UnWrapGui, View};

pub fn thick_row(row_index: usize) -> bool {
    row_index % 6 == 0
}

#[derive(Debug, PartialEq, Eq, Default)]
pub(super) enum ShowTable {
    #[default]
    Source,
    Target,
    Output,
}

#[derive(Debug, Default)]
pub(crate) struct CenterWindow {
    pub(super) output: RefCell<OutputTable>,
    pub(super) input_source: RefCell<InputTabel>,
    pub(super) input_target: RefCell<InputTabel>,
    pub(super) algoritma: usize,
    pub(super) show_table: ShowTable,
    pub(super) ignore_num: bool,
}

impl CenterWindow {
    pub fn ui(&mut self, ctx: &eframe::egui::Context) {
        eframe::egui::SidePanel::left("panel_config_left")
            .resizable(true)
            .frame(Frame::none().inner_margin(Margin::symmetric(10f32, 10f32)))
            .show(ctx, |ui| self.side_bar(ui));
        eframe::egui::CentralPanel::default()
            .frame(Frame::none().inner_margin(Margin::symmetric(10f32, 10f32)))
            .show(ctx, |ui| match self.show_table {
                ShowTable::Source => self.input_source.get_mut().ui(ui),
                ShowTable::Target => self.input_target.get_mut().ui(ui),
                ShowTable::Output => self.output.get_mut().ui(ui),
            });
    }
    pub fn compare(&mut self) {
        let src = self.input_source.borrow();
        let target = self.input_target.borrow();
        if src.is_opened() && target.is_opened() {
            let (_src, _tgt) = Comparison::run(
                match self.algoritma {
                    0 => similar::Algorithm::Myers,
                    1 => similar::Algorithm::Patience,
                    2 => similar::Algorithm::Lcs,
                    _ => Err(DpdError::Processing(
                        "Indexing on Algoritm Chosen".to_owned(),
                    ))
                    .unwrap_gui(),
                },
                src.data.selected_data.as_ref(),
                target.data.selected_data.as_ref(),
                &src.data.sheets[src.idx_sheet].clone(),
                &src.data.file,
                &target.data.file,
                self.ignore_num
            )
            .unwrap_gui()
            .get_data();

            let output = self.output.get_mut();
            output.set_src(_src);
            output.set_tgt(_tgt);
            self.show_table = ShowTable::Output;
        } else {
        }
    }
    fn side_bar(&mut self, uiwin: &mut eframe::egui::Ui) {
        uiwin.with_layout(Layout::top_down_justified(Align::Center), |ui| {
            ui.group(|ui| {
                use ShowTable::{Output, Source, Target};
                ui.radio_value(
                    &mut self.show_table,
                    Source,
                    RichText::new("TABEL SUMBER").strong().size(18f32),
                );
                ui.radio_value(
                    &mut self.show_table,
                    Target,
                    RichText::new("TABEL TARGET").strong().size(18f32),
                );
                ui.radio_value(
                    &mut self.show_table,
                    Output,
                    RichText::new("TABEL OUTPUT").strong().size(18f32),
                );
            });
            ui.separator();
            ComboBox::from_label("Algoritma").show_index(
                ui,
                &mut self.algoritma,
                3,
                |idx| match idx {
                    0 => "Myers".to_owned(),
                    1 => "Patience".to_owned(),
                    2 => "Lcs".to_owned(),
                    _ => Default::default(),
                },
            );
            ui.separator();
            ui.add_enabled_ui(self.is_ready_compare(), |ui| {
                ui.checkbox(&mut self.ignore_num, "Ignore Numbering");
                ui.separator();
                if ui
                    .add(
                        eframe::egui::Button::new(
                            RichText::new("COMPARE INPUT")
                                .strong()
                                .size(18f32)
                                .color(Color32::WHITE),
                        )
                        .fill(Color32::DARK_GREEN),
                    )
                    .on_disabled_hover_text("Open 2 input in Tab input, before you want to compare")
                    .clicked()
                {
                    self.compare()
                }
            });
        });
        uiwin.separator();
        self.output.borrow_mut().on_sidebar(uiwin);
        uiwin.separator();
        uiwin.with_layout(
            Layout::bottom_up(Align::Center).with_cross_justify(true),
            |ui| {
                ui.label("© CopyRights Rizal Achmad Pahlevi");
                ui.separator();
                if ui
                    .add(
                        eframe::egui::Button::new(
                            RichText::new("CLOSE ALL")
                                .size(18f32)
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .fill(Color32::DARK_RED),
                    )
                    .on_hover_text("Close All opened Table and Input")
                    .clicked()
                {
                    self.close_current();
                }
                ui.separator();
                if ui
                    .add(
                        eframe::egui::Button::new(
                            RichText::new("SWAP INPUT")
                                .size(18f32)
                                .strong()
                                .color(Color32::BLACK),
                        )
                        .fill(Color32::YELLOW),
                    )
                    .on_hover_text("Swap Source Table with Target Table")
                    .clicked()
                {
                    std::mem::swap(&mut self.input_target, &mut self.input_source);
                    self.input_source.get_mut().refresh();
                    self.input_target.get_mut().refresh();
                }
            },
        );
    }

    pub fn is_ready_compare(&self) -> bool {
        self.input_source.borrow().is_opened() && self.input_target.borrow().is_opened()
    }
    pub fn close_current(&mut self) {
        self.output.get_mut().clear();
        self.input_source.get_mut().clear();
        self.input_target.get_mut().clear();
    }
}
