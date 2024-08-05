use crate::installer_stack_page;
use crate::config;
use gtk::{prelude::*, glib as glib, Justification};
use glib::clone;
pub fn welcome_page(window: &adw::ApplicationWindow, main_carousel: &adw::Carousel) {
    let welcome_page = installer_stack_page::InstallerStackPage::new();
    welcome_page.set_page_title(t!("welcome"));
    welcome_page.set_page_subtitle(t!("welcome_to_pikaos"));
    welcome_page.set_page_icon(config::DISTRO_ICON);
    welcome_page.set_back_visible(false);
    welcome_page.set_next_visible(false);

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_start(15)
        .margin_end(15)
        .margin_top(15)
        .margin_bottom(15)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .vexpand(true)
        .hexpand(true)
        .build();

    content_box.add_css_class("linked");

    let live_media_button = gtk::Button::builder()
        .icon_name("drive-optical")
        .label(t!("use_pikaos_in_live_media"))
        .build();

    let install_media_button = gtk::Button::builder()
        .icon_name("drive-harddisk")
        .label(t!("install_distro_to_system"))
        .build();

    install_media_button.connect_clicked(
        clone!(
            #[weak]
            main_carousel,
            move |_|
            main_carousel.scroll_to(&main_carousel.nth_page(1), true)
        )
    );

    live_media_button.connect_clicked(
        clone!(
            #[weak]
            window,
            move |_|
            window.close()
        )
    );

    content_box.append(&install_media_button);
    content_box.append(&live_media_button);

    welcome_page.set_child_widget(&content_box);

    main_carousel.append(&welcome_page);
}