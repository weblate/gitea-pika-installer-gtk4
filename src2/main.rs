
// Use libraries
use std::env;
use crate::build_ui::build_ui;
use adw::prelude::*;
use adw::*;
use gdk::Display;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::{CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};

mod config;
use config::{APP_ID};

// Init translations for current crate.
#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en_US");


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
    let current_locale = match env::var_os("LANG") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$LANG is not set"),
    };
    rust_i18n::set_locale(current_locale.strip_suffix(".UTF-8").unwrap());
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
        app.connect_activate(build_ui);
    });

    let instance = SingleInstance::new(APP_ID).unwrap();
    assert!(instance.is_single());

    application.run();
}
