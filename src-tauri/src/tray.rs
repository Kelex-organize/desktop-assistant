use tauri::{
    menu::{Menu, MenuEvent, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle,
};

use crate::windows::show_launcher;

/// Id of the "Show" menu item.
pub const SHOW_ID: &str = "show";
/// Id of the "Quit" menu item.
pub const QUIT_ID: &str = "quit";

/// Creates the tray icon with its menu (Show, Quit) and hooks up its events.
pub fn setup(app: &AppHandle) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, SHOW_ID, "Show", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, QUIT_ID, "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &quit])?;

    let mut builder = TrayIconBuilder::new()
        .tooltip("Kai")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(on_menu_event)
        .on_tray_icon_event(on_tray_icon_event);

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    builder.build(app)?;

    Ok(())
}

/// Reacts to a click on a menu item. Use `event.id()` to know which one.
fn on_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id().as_ref() {
        SHOW_ID => {
            if let Err(error) = show_launcher(app) {
                eprintln!("could not show the launcher window: {error}");
            }
        }
        QUIT_ID => {
            app.exit(0);
        }
        _ => {}
    }
}

/// Reacts to a mouse event on the tray icon itself.
fn on_tray_icon_event(tray: &TrayIcon, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
    } = event
    {
        let app = tray.app_handle();
        if let Err(error) = show_launcher(app) {
            eprintln!("could not show the launcher window: {error}");
        }
    }
}
