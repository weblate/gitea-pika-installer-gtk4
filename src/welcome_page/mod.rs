use crate::config;
use crate::installer_stack_page;
use glib::clone;
use gtk::{glib, prelude::*};
pub fn welcome_page(window: &adw::ApplicationWindow, main_carousel: &adw::Carousel) {
    let welcome_page = installer_stack_page::InstallerStackPage::new();
    welcome_page.set_page_title(t!("welcome_page_title"));
    welcome_page.set_page_subtitle(t!("welcome_page_subtitle"));
    welcome_page.set_page_icon(config::DISTRO_ICON);
    welcome_page.set_back_visible(false);
    welcome_page.set_next_visible(false);

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .vexpand(true)
        .hexpand(true)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .homogeneous(true)
        .build();

    content_box.add_css_class("linked");

    let live_media_button = gtk::Button::builder()
        .icon_name("drive-optical")
        .label(t!("live_media_button_label"))
        .build();

    let install_media_button = gtk::Button::builder()
        .icon_name("drive-harddisk")
        .label(t!("install_media_button_label"))
        .build();

    install_media_button.connect_clicked(clone!(
        #[weak]
        main_carousel,
        move |_| main_carousel.scroll_to(&main_carousel.nth_page(1), true)
    ));

    live_media_button.connect_clicked(clone!(
        #[weak]
        window,
        move |_| window.close()
    ));

    content_box.append(&install_media_button);
    content_box.append(&live_media_button);

    welcome_page.set_child_widget(&content_box);

    main_carousel.append(&welcome_page);
}
