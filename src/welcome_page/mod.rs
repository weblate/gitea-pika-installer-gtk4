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

    let live_media_button_icon = gtk::Image::builder()
        .icon_name("drive-optical")
        .margin_end(2)
        .halign(gtk::Align::Start)
        .build();

    let live_media_button_label = gtk::Label::builder()
        .label(t!("live_media_button_label"))
        .halign(gtk::Align::Center)
        .hexpand(true)
        .build();

    let live_media_button_child_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    live_media_button_child_box.append(&live_media_button_icon);
    live_media_button_child_box.append(&live_media_button_label);

    let install_media_button_icon = gtk::Image::builder()
        .icon_name("drive-harddisk")
        .margin_end(2)
        .halign(gtk::Align::Start)
        .build();

    let install_media_button_label = gtk::Label::builder()
        .label(t!("install_media_button_label"))
        .halign(gtk::Align::Center)
        .hexpand(true)
        .build();

    let install_media_button_child_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    install_media_button_child_box.append(&install_media_button_icon);
    install_media_button_child_box.append(&install_media_button_label);

    let media_labels_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Both);
    let media_icons_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Both);

    media_labels_size_group.add_widget(&live_media_button_label);
    media_labels_size_group.add_widget(&install_media_button_label);

    media_icons_size_group.add_widget(&live_media_button_icon);
    media_icons_size_group.add_widget(&install_media_button_icon);

    let live_media_button = gtk::Button::builder()
        .child(&live_media_button_child_box)
        .build();

    let install_media_button = gtk::Button::builder()
        .child(&install_media_button_child_box)
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
