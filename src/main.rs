mod dpdcmpexcel;
mod gui;

fn main() -> dpdcmpexcel::errors::DpdResult<()> {
    let (source, target, isgui) = getargs();
    if !isgui {
        run_without_ui(source.unwrap(), target.unwrap())?;
    } else {
        run_with_ui()?;
    }
    Ok(())
}

fn getargs() -> (Option<String>, Option<String>, bool) {
    match (std::env::args().nth(1), std::env::args().nth(2)) {
        (Some(o), Some(b)) => (Some(o), Some(b), false),
        (Some(_), None) => (None, None, true),
        (None, Some(_)) => (None, None, true),
        (None, None) => (None, None, true),
    }
}

fn run_without_ui(source: String, target: String) -> dpdcmpexcel::errors::DpdResult<()> {
    Err(dpdcmpexcel::errors::DpdError::Processing(format!("todo! run_without_ui implementation {}{}", source, target)))
}

fn run_with_ui() -> dpdcmpexcel::errors::DpdResult<()> {
    let icon = image::load_from_memory_with_format(
        include_bytes!("assets/logo-dispendik-piala.png"),
        image::ImageFormat::Png,
    )
    .expect("Failed to load icon")
    .to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();

    eframe::run_native(
        "DispendikCompExcel",
        eframe::NativeOptions {
            drag_and_drop_support: true,
            icon_data: Some(eframe::IconData {
                rgba: icon.into_raw(),
                width: icon_width,
                height: icon_height,
            }),
            default_theme: eframe::Theme::Dark,
            ..Default::default()
        },
        Box::new(|_| Box::new(gui::Application::default())),
    );
    Ok(())
}
