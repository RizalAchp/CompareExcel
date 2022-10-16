pub mod inputtabel;
pub mod mainwindow;
pub mod outputtabel;

use crate::dpdcmpexcel::errors::DpdResult;

use self::mainwindow::CenterWindow;

#[cfg(linux)]
pub const HOME: &'static str = env!("HOME");
#[cfg(windows)]
pub const HOME: Option<&str> = option_env!("USERPROFILE");

#[macro_export]
macro_rules! exec_async {
    ($f:tt) => {
        std::thread::spawn(move || {
            futures::executor::block_on(async move $f);
        });
    };
}

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
#[derive(Debug)]
pub(crate) struct Application {
    pub(super) center_window: CenterWindow,
    pub(super) allowed_to_close: bool,
    pub(super) show_confirmation_dialog: bool,
    pub(super) show_error_message: bool,
    pub(super) show_help_message: bool,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            center_window: CenterWindow::default(),
            allowed_to_close: false,
            show_confirmation_dialog: false,
            show_error_message: false,
            show_help_message: false,
        }
    }
}

impl Application {
    #[allow(unused)]
    pub(crate) fn help_window(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {}

    #[allow(unused)]
    pub(crate) fn bar_contents(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {
        eframe::egui::widgets::global_dark_light_mode_switch(ui);
        ui.separator();
        ui.with_layout(
            eframe::egui::Layout::left_to_right(eframe::egui::Align::LEFT),
            |ui| {
            },
        );
        ui.with_layout(
            eframe::egui::Layout::right_to_left(eframe::egui::Align::RIGHT),
            |ui| {
                if ui.button("help").clicked() {
                    self.show_help_message = !self.show_help_message;
                }
                ui.separator();
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
        ctx.request_repaint_after(std::time::Duration::from_secs_f32(1.0f32));
        eframe::egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
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
                        if ui.button("Yes").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }
                        if ui.button("No").clicked() {
                            self.show_confirmation_dialog = false;
                        }
                    });
                });
        }
    }

    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }
}

pub enum Message {
    FileOpen(DpdResult<crate::dpdcmpexcel::CmpData>),
    #[allow(unused)]
    ReturnDialog(bool),
    #[allow(unused)]
    IgnoredResult(Option<()>),
    // Other messages
}
trait UnWrapGui<T> {
    fn unwrap_gui(self) -> T;
}

impl<T: Default + ?Sized> UnWrapGui<T> for DpdResult<T> {
    fn unwrap_gui(self) -> T {
        match self {
            Ok(k) => k,
            Err(e) => {
                exec_async!({
                    rfd::MessageDialog::new()
                        .set_level(rfd::MessageLevel::Error)
                        .set_title("Got Error!")
                        .set_buttons(rfd::MessageButtons::Ok)
                        .set_description(&e.to_string())
                        .show()
                });

                Default::default()
            }
        }
    }
}

trait DisplayGui {
    fn display_gui_text(&self) -> eframe::egui::RichText;
}
impl DisplayGui for Vec<String> {
    fn display_gui_text(&self) -> eframe::egui::RichText {
        use std::fmt::Write;
        let mut buf = String::new();
        for item in self {
            write!(buf, "{}..", item).ok();
        }
        buf.into()
    }
}

impl DisplayGui for String {
    #[inline]
    fn display_gui_text(&self) -> eframe::egui::RichText {
        use eframe::egui::RichText;
        if let Some(filename) = std::path::PathBuf::from(self).file_name() {
            RichText::from(filename.to_string_lossy())
                .underline()
                .color(eframe::egui::epaint::Color32::GREEN)
        } else {
            RichText::default()
        }
    }
}
