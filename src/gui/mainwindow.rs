use std::cell::RefCell;

use crate::dpdcmpexcel::compares::Comparison;

use super::inputtabel::InputTabel;
use super::outputtabel::OutputTable;
use super::View;

pub fn thick_row(row_index: usize) -> bool {
    row_index % 6 == 0
}

#[derive(PartialEq, Eq)]
pub(super) enum ShowTable {
    Source,
    Target,
    Output,
}

pub(crate) struct CenterWindow {
    pub(super) output: RefCell<OutputTable>,
    pub(super) input_source: RefCell<InputTabel>,
    pub(super) input_target: RefCell<InputTabel>,

    pub(super) datacompare: Option<Comparison>,
    pub(super) show_table: ShowTable,
}

impl Default for CenterWindow {
    fn default() -> Self {
        Self {
            output: RefCell::new(Default::default()),
            input_source: RefCell::new(Default::default()),
            input_target: RefCell::new(Default::default()),
            datacompare: None,
            show_table: ShowTable::Source,
        }
    }
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
    fn side_bar(&mut self, uiwin: &mut eframe::egui::Ui) {
        uiwin.group(|ui| {
            ui.radio_value(&mut self.show_table, ShowTable::Source, "Sumber");
            ui.radio_value(&mut self.show_table, ShowTable::Target, "Target");
            ui.radio_value(&mut self.show_table, ShowTable::Output, "Perbedaan");
        });
    }

    pub fn close_current(&mut self) {
        self.datacompare = None;
        self.input_source.get_mut().clear();
        self.input_target.get_mut().clear();
        self.output.get_mut().clear();
    }
}
