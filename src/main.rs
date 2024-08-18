use gdk::Display;
use gtk::{gdk, glib, prelude::*, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};
use std::env;
mod build_ui;
mod config;
mod efi_error_page;
mod installer_stack_page;

//

mod automatic_partitioning_page;
mod drive_mount_row;
mod eula_page;
mod keyboard_page;
mod language_page;
mod manual_partitioning_page;
mod partitioning_page;
mod timezone_page;
mod welcome_page;

#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en_US");

fn main() -> glib::ExitCode {
    let current_locale = match env::var_os("LANG") {
        Some(v) => v
            .into_string()
            .unwrap()
            .chars()
            .take_while(|&ch| ch != '.')
            .collect::<String>(),
        None => panic!("$LANG is not set"),
    };
    rust_i18n::set_locale(&current_locale);

    let app = adw::Application::builder()
        .application_id(config::APP_ID)
        .build();

    app.connect_startup(|app| {
        load_css();
        app.connect_activate(build_ui::build_ui);
    });

    // Run the application
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
