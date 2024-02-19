// Use libraries
use crate::build_ui::build_ui;
use adw::prelude::*;
use adw::*;
use gdk::Display;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::*;

mod config;
use config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR};
use gettextrs::{gettext, LocaleCategory};

mod automatic_partitioning;
mod build_ui;
mod done_page;
mod drive_mount_row;
mod efi_error_page;
mod eula_page;
mod install_page;
mod keyboard_page;
mod language_page;
mod manual_partitioning;
mod partitioning_page;
mod save_window_size;
mod timezone_page;
mod welcome_page;

/// main function
fn main() {
    let application = adw::Application::new(Some(APP_ID), Default::default());
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
        // Prepare i18n
        gettextrs::setlocale(LocaleCategory::LcAll, "");
        gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR)
            .expect("Unable to bind the text domain");
        gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");
        // Fallback if no translation present
        if gettext("pikaos_installer") == "pikaos_installer" {
            println!("Warning: Current LANG is not supported, using fallback Locale.");
            gettextrs::setlocale(LocaleCategory::LcAll, "en_US.UTF8");
        }

        app.connect_activate(build_ui);
    });

    application.run();
}
