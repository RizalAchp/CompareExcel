pub mod inputtabel;
pub mod mainwindow;
pub mod outputtabel;

use crate::dpdcmpexcel::{compares::CmpData, errors::DpdResult};

use self::mainwindow::CenterWindow;

pub trait Window {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &eframe::egui::Context, open: &mut bool);
}
pub trait View {
    fn ui(&mut self, ui: &mut eframe::egui::Ui);
}

pub trait TableData {
    fn pass_data(d: &Vec<Vec<String>>);
}

#[allow(unused)]
#[derive(Default)]
pub(crate) struct Application {
    pub(super) center_window: CenterWindow,
    pub(super) allowed_to_close: bool,
    pub(super) show_confirmation_dialog: bool,
    pub(super) show_error_message: bool,
}

impl Application {
    #[allow(unused)]
    pub(crate) fn bar_contents(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {
        eframe::egui::widgets::global_dark_light_mode_switch(ui);
        ui.separator();
        ui.with_layout(
            eframe::egui::Layout::left_to_right(eframe::egui::Align::LEFT),
            |ui| {
                if ui.button("Clear All").clicked() {
                    self.center_window.close_current();
                };
                ui.separator();
                if ui.button("Swap Input").clicked() {
                    std::mem::swap(
                        &mut self.center_window.input_target,
                        &mut self.center_window.input_source,
                    );
                    self.center_window.input_source.get_mut().refresh();
                    self.center_window.input_target.get_mut().refresh();
                };
            },
        );
        ui.with_layout(
            eframe::egui::Layout::right_to_left(eframe::egui::Align::RIGHT),
            |ui| {
                eframe::egui::warn_if_debug_build(ui);
            },
        );
    }
}

impl eframe::App for Application {
    fn clear_color(&self, visuals: &eframe::egui::Visuals) -> eframe::egui::Rgba {
        visuals.window_fill().into()
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            eframe::egui::trace!(ui);
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ui, frame);
            });
        });
        self.center_window.ui(ctx);

        if self.show_confirmation_dialog {
            // Show confirmation dialog:
            let painter = ctx.layer_painter(eframe::egui::LayerId::new(
                eframe::egui::Order::Background,
                eframe::egui::Id::new("confirmation_quit"),
            ));

            let screen_rect = ctx.input().screen_rect();
            painter.rect_filled(
                screen_rect,
                0.0,
                eframe::egui::Color32::from_black_alpha(192),
            );
            eframe::egui::Window::new("Do you want to quit?")
                .anchor(eframe::egui::Align2::CENTER_CENTER, [-50f32, 0f32])
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Hoyeahh!").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }
                        if ui.button("Tidak Jadi").clicked() {
                            self.show_confirmation_dialog = false;
                        }
                    });
                });
        }
        ctx.request_repaint_after(std::time::Duration::from_secs_f32(1.0f32));
    }

    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }
}

trait UnWrapGui<T> {
    fn unwrap_gui(self) -> Option<T>;
}

#[macro_export]
macro_rules! unwrap_gui_impl {
    ($t:ty) => {
        impl UnWrapGui<$t> for DpdResult<$t> {
            #[inline]
            fn unwrap_gui(self) -> Option<$t> {
                match self {
                    Ok(k) => Some(k),
                    Err(e) => {
                        rfd::MessageDialog::new()
                            .set_level(rfd::MessageLevel::Error)
                            .set_title("Got Error!")
                            .set_buttons(rfd::MessageButtons::Ok)
                            .set_description(&e.to_string())
                            .show();
                        None
                    }
                }
            }
        }
    };
}

unwrap_gui_impl!(CmpData);
unwrap_gui_impl!((Vec<Vec<String>>, (usize, usize)));
unwrap_gui_impl!(Vec<Vec<String>>);
unwrap_gui_impl!(());
