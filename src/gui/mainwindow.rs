use std::cell::RefCell;

use crate::dpdcmpexcel::compares::Comparison;

use super::inputtabel::InputTabel;
use super::outputtabel::OutputTable;
use super::{UnWrapGui, View};

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

    pub(super) datacompare: RefCell<Comparison>,
    pub(super) show_table: ShowTable,
}

impl Default for CenterWindow {
    fn default() -> Self {
        Self {
            output: RefCell::new(Default::default()),
            input_source: RefCell::new(Default::default()),
            input_target: RefCell::new(Default::default()),
            datacompare: RefCell::new(Default::default()),
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
        let src = self.input_source.borrow();
        let target = self.input_target.borrow();
        if !src.hovered_inactive() && !target.hovered_inactive() {
            if uiwin.button("Start Compare").clicked() {
                let src_data = src.selected_data.as_ref();
                let target_data = target.selected_data.as_ref();
                let sheets = if let Some(s) = &src.data {
                    s.sheets[src.selected_idx].clone()
                } else {
                    "".to_owned()
                };
                self.datacompare
                    .get_mut()
                    .run(src_data, target_data, &sheets)
                    .unwrap_gui();
                self.output.get_mut().data = Some(self.datacompare.take().0);
                self.show_table = ShowTable::Output;
            }
        }
        uiwin.group(|ui| {
            ui.radio_value(&mut self.show_table, ShowTable::Source, "Sumber");
            ui.radio_value(&mut self.show_table, ShowTable::Target, "Target");
            ui.radio_value(&mut self.show_table, ShowTable::Output, "Perbedaan");
        });
    }

    pub fn close_current(&mut self) {
        self.datacompare.get_mut().0.clear();
        self.output.get_mut().clear();
        self.input_source.get_mut().clear();
        self.input_target.get_mut().clear();
    }
}
