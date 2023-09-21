
use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::{Label, Flex};
use sys_info;

fn build_ui() -> impl Widget<()> {
    // HWID Info
    let os_type = sys_info::os_type().unwrap_or_else(|_| "Unknown".to_string());
    let os_release = sys_info::os_release().unwrap_or_else(|_| "Unknown".to_string());
    let os_version = sys_info::os_version().unwrap_or_else(|_| "Unknown".to_string());

    // Net Info
    let ip = sys_info::ip().unwrap_or_else(|_| "Unknown".to_string());
    let hostname = sys_info::hostname().unwrap_or_else(|_| "Unknown".to_string());

    // Disk Space
    let disk = sys_info::disk_info().unwrap();
    let total_space = disk.total;
    let free_space = disk.free;
    let used_space = total_space - free_space;

    // CPU Load
    let cpu_load = sys_info::loadavg().unwrap();
    let one_min_load = cpu_load.one;
    let five_min_load = cpu_load.five;
    let fifteen_min_load = cpu_load.fifteen;

    // Memory Info
    let mem_info = sys_info::mem_info().unwrap();
    let total_memory = mem_info.total;
    let free_memory = mem_info.free;

    // Boot Time
    let boot_time = sys_info::boottime().unwrap();
    let boot_time_secs = boot_time.tv_sec;

    // Building the UI
    let root = Flex::column()
        .with_child(Label::new(format!("OS Type: {}", os_type)))
        .with_child(Label::new(format!("OS Release: {}", os_release)))
        .with_child(Label::new(format!("OS Version: {}", os_version)))
        .with_child(Label::new(format!("IP: {}", ip)))
        .with_child(Label::new(format!("Hostname: {}", hostname)))
        .with_child(Label::new(format!("Total Space: {}", total_space)))
        .with_child(Label::new(format!("Used Space: {}", used_space)))
        .with_child(Label::new(format!("Free Space: {}", free_space)))
        .with_child(Label::new(format!("1 Min CPU Load: {}", one_min_load)))
        .with_child(Label::new(format!("5 Min CPU Load: {}", five_min_load)))
        .with_child(Label::new(format!("15 Min CPU Load: {}", fifteen_min_load)))
        .with_child(Label::new(format!("Total Memory: {}", total_memory)))
        .with_child(Label::new(format!("Free Memory: {}", free_memory)))
        .with_child(Label::new(format!("Boot Time (secs since epoch): {}", boot_time_secs)))
        .with_spacer(20.0);

    root
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_ui())
        .title("System Info App")
        .window_size((600.0, 600.0));

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(())

    Ok(())
}
