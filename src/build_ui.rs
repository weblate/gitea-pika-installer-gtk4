use std::path::Path;
use gtk::{prelude::*, glib as glib, gio as gio};
use crate::{efi_error_page, welcome_page, language_page, eula_page, keyboard_page, timezone_page, partitioning_page};

pub fn build_ui(app: &adw::Application) {
    glib::set_prgname(Some("pikaos_installer"));
    glib::set_application_name(&t!("pikaos_installer"));

    let carousel = adw::Carousel::builder()
        .allow_long_swipes(false)
        .allow_mouse_drag(false)
        .allow_scroll_wheel(false)
        .interactive(false)
        .vexpand(true)
        .hexpand(true)
        .build();

    let carousel_indicator = adw::CarouselIndicatorDots::builder()
        .carousel(&carousel)
        .build();

    let window_headerbar = adw::HeaderBar::builder()
        .show_start_title_buttons(true)
        .title_widget(&carousel_indicator)
        .build();

    let toolbarview = adw::ToolbarView::builder()
        .top_bar_style(adw::ToolbarStyle::Flat)
        .content(&carousel)
        .build();

    toolbarview.add_top_bar(&window_headerbar);

    let window = adw::ApplicationWindow::builder()
        .title(t!("pikaos_installer"))
        .application(app)
        .icon_name("calamares")
        .width_request(700)
        .height_request(500)
        .default_width(700)
        .default_height(500)
        .deletable(false)
        .content(&toolbarview)
        .startup_id("pika-installer-gtk4")
        .build();

    match Path::new("/sys/firmware/efi/efivars").exists() {
        true => welcome_page::welcome_page(&window, &carousel),
        _ => efi_error_page::efi_error_page(&window, &carousel)
    }

    let language_selection_text_buffer = gtk::TextBuffer::builder().build();
    let keymap_base_selection_text_buffer = gtk::TextBuffer::builder().build();
    let keymap_varient_selection_text_buffer = gtk::TextBuffer::builder().build();
    let timezone_selection_text_buffer = gtk::TextBuffer::builder().build();
    let partition_method_type_buffer= gtk::TextBuffer::builder().build();
    let partition_method_automatic_target_buffer= gtk::TextBuffer::builder().build();
    let partition_method_automatic_luks_buffer = gtk::TextBuffer::builder().build();
    let partition_method_automatic_ratio_buffer= gtk::TextBuffer::builder().build();
    let partition_method_automatic_seperation_buffer= gtk::TextBuffer::builder().build();

    let language_changed_action = gio::SimpleAction::new("lang-changed", None);

    language_page::language_page(&carousel, &language_selection_text_buffer, &language_changed_action);

    eula_page::eula_page(&carousel, &language_changed_action);

    keyboard_page::keyboard_page(&carousel, &keymap_base_selection_text_buffer, &keymap_varient_selection_text_buffer, &language_changed_action);

    timezone_page::timezone_page(&carousel, &timezone_selection_text_buffer, &language_changed_action);

    partitioning_page::partitioning_page(
    &carousel,
    &partition_method_type_buffer,
    &partition_method_automatic_target_buffer,
    &partition_method_automatic_luks_buffer,
    &partition_method_automatic_ratio_buffer,
    &partition_method_automatic_seperation_buffer,
    &language_changed_action);

    window.present()
}