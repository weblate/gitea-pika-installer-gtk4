use glib::clone;
use gtk::{glib, prelude::*, Justification};
pub fn efi_error_page(window: &adw::ApplicationWindow, main_carousel: &adw::Carousel) {
    let efi_error_main_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let efi_error_header_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let efi_error_header_text = gtk::Label::builder()
        .label(t!("efi_error_header_text_label"))
        .halign(gtk::Align::End)
        .hexpand(true)
        .wrap(true)
        .justify(Justification::Center)
        .width_chars(20)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(5)
        .build();
    efi_error_header_text.add_css_class("header_sized_text");

    let efi_error_header_icon = gtk::Image::builder()
        .icon_name("emblem-error")
        .halign(gtk::Align::Start)
        .hexpand(true)
        .pixel_size(78)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(0)
        .margin_end(15)
        .build();

    let efi_error_selection_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_bottom(15)
        .margin_top(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    let efi_error_text = gtk::Label::builder()
        .vexpand(true)
        .hexpand(true)
        .label(t!("efi_error_text_label"))
        .wrap(true)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    efi_error_text.add_css_class("big_error_text");

    let exit_button = gtk::Button::builder()
        .label(t!("exit_button_label"))
        .vexpand(true)
        .hexpand(true)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    efi_error_header_box.append(&efi_error_header_text);
    efi_error_header_box.append(&efi_error_header_icon);
    efi_error_main_box.append(&efi_error_header_box);
    efi_error_main_box.append(&efi_error_selection_box);
    efi_error_selection_box.append(&efi_error_text);
    efi_error_selection_box.append(&exit_button);
    efi_error_header_box.append(&efi_error_header_text);
    efi_error_header_box.append(&efi_error_header_icon);
    efi_error_main_box.append(&efi_error_header_box);
    efi_error_main_box.append(&efi_error_selection_box);
    main_carousel.append(&efi_error_main_box);

    exit_button.connect_clicked(clone!(
        #[weak]
        window,
        move |_| window.close()
    ));
}
