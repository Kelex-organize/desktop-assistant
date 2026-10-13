use tauri::{
    menu::MenuEvent,
    tray::{TrayIcon, TrayIconEvent},
    AppHandle,
};

/// Id of the "Show" menu item.
pub const SHOW_ID: &str = "show";
/// Id of the "Quit" menu item.
pub const QUIT_ID: &str = "quit";

/// Creates the tray icon with its menu (Show, Quit) and hooks up its events.
// TODO(#7): implement, then remove the `allow`. Right now it creates nothing.
#[allow(unused_variables)]
pub fn setup(app: &AppHandle) -> tauri::Result<()> {
    Ok(())
}

/// Reacts to a click on a menu item. Use `event.id()` to know which one.
// TODO(#7): implement, then remove the `allow`.
#[allow(unused_variables, dead_code)]
fn on_menu_event(app: &AppHandle, event: MenuEvent) {
    todo!()
}

/// Reacts to a mouse event on the tray icon itself.
// TODO(#7): implement, then remove the `allow`.
#[allow(unused_variables, dead_code)]
fn on_tray_icon_event(tray: &TrayIcon, event: TrayIconEvent) {
    todo!()
}
