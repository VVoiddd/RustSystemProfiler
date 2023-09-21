
use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::{Label, Flex};
use sys_info;

fn build_ui() -> impl Widget<()> {
    let root = Flex::column()
        .with_child(Label::new(format!("OS Type: {}", sys_info::os_type().unwrap_or_else(|_| "Unknown".to_string()))))
        .with_child(Label::new(format!("OS Release: {}", sys_info::os_release().unwrap_or_else(|_| "Unknown".to_string()))))
        .with_child(Label::new(format!("OS Version: {}", sys_info::os_version().unwrap_or_else(|_| "Unknown".to_string()))))
        .with_child(Label::new(format!("IP: {}", sys_info::ip().unwrap_or_else(|_| "Unknown".to_string()))))
        .with_child(Label::new(format!("Hostname: {}", sys_info::hostname().unwrap_or_else(|_| "Unknown".to_string()))))
        .with_spacer(20.0);

    root
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_ui())
        .title("System Info App")
        .window_size((400.0, 400.0));
    
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(())?;
    
    Ok(())
}
