
// Use libraries
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::prelude::*;
use gtk::*;
use adw::prelude::*;
use adw::*;
use glib::*;
use gdk::Display;
use gtk::subclass::layout_child;
mod build_ui;
use crate::build_ui::build_ui;
mod save_window_size;
mod welcome_page;
mod efi_error_page;
mod language_page;
mod eula_page;
mod timezone_page;
mod keyboard_page;
mod partitioning_page;
mod install_page;
mod done_page;
use crate::save_window_size::save_window_size;
use crate::welcome_page::welcome_page;
use crate::efi_error_page::efi_error_page;
use crate::language_page::language_page;
use crate::eula_page::eula_page;
use crate::timezone_page::timezone_page;
use crate::keyboard_page::keyboard_page;
use crate::partitioning_page::partitioning_page;
use crate::install_page::install_page;
use crate::done_page::done_page;

/// main function
fn main() {
    let application = adw::Application::new(Some("com.github.pikaos-linux.pikainstallergtk4"), Default::default());
    application.connect_startup(|app| {
        // The CSS "magic" happens here.
        let provider = CssProvider::new();
        provider.load_from_string(include_str!("style.css"));
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::style_context_add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        app.connect_activate(build_ui);
    });
    application.run();
}
