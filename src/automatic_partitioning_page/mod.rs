use adw::gio;
use crate::installer_stack_page;
use gtk::{prelude::*, glib as glib};
use glib::{clone, closure_local};

pub fn automatic_partitioning_page(
    main_carousel: &adw::Carousel,
    language_changed_action: &gio::SimpleAction
) {
    let automatic_partitioning_page = installer_stack_page::InstallerStackPage::new();
    automatic_partitioning_page.set_page_icon("builder");
    automatic_partitioning_page.set_back_visible(true);
    automatic_partitioning_page.set_next_visible(true);
    automatic_partitioning_page.set_back_sensitive(true);
    automatic_partitioning_page.set_next_sensitive(false);

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();

    automatic_partitioning_page.connect_closure(
        "back-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            move |_automatic_partitioning_page: installer_stack_page::InstallerStackPage|
            {
                    main_carousel.scroll_to(&main_carousel.nth_page(0), true)
            }
        )
    );

    main_carousel.append(&automatic_partitioning_page);
}
