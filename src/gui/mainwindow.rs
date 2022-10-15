use std::cell::RefCell;

use eframe::egui::ComboBox;

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
}

impl CenterWindow {
    pub fn ui(&mut self, ctx: &eframe::egui::Context) {
        eframe::egui::SidePanel::left("panel_config_left")
            .resizable(true)
            .show(ctx, |ui| self.side_bar(ui));
        eframe::egui::CentralPanel::default().show(ctx, |ui| match self.show_table {
            ShowTable::Source => self.input_source.get_mut().ui(ui),
            ShowTable::Target => self.input_target.get_mut().ui(ui),
            ShowTable::Output => self.output.get_mut().ui(ui),
        });
    }
    pub fn compare(&mut self) {
        let src = self.input_source.borrow();
        let target = self.input_target.borrow();
        if !src.hovered_inactive() && !target.hovered_inactive() {
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
                &src.data.sheets[src.selected_idx].clone(),
                &src.data.file,
                &target.data.file,
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
        uiwin.group(|ui| {
            use ShowTable::{Output, Source, Target};
            ui.radio_value(&mut self.show_table, Source, "EXCEL SUMBER");
            ui.radio_value(&mut self.show_table, Target, "EXCEL TARGET");
            ui.radio_value(&mut self.show_table, Output, "PERBEDAAN");
        });
        uiwin.separator();
        ComboBox::from_label("Algoritma").show_index(
            uiwin,
            &mut self.algoritma,
            3,
            |idx| match idx {
                0 => "Myers".to_owned(),
                1 => "Patience".to_owned(),
                2 => "Lcs".to_owned(),
                _ => Default::default(),
            },
        );
        uiwin.separator();
        if uiwin.button("Start Compare").clicked() {
            self.compare()
        }
    }

    pub fn close_current(&mut self) {
        self.output.get_mut().clear();
        self.input_source.get_mut().clear();
        self.input_target.get_mut().clear();
    }
}
