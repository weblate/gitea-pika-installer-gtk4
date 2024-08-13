use adw::gio;
use crate::installer_stack_page;
use gtk::{prelude::*, glib as glib};
use crate::partitioning_page::{get_block_devices};
use adw::{prelude::*};
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

    //

    let devices_selection_expander_row = adw::ExpanderRow::builder()
        .name("status:disk=none,")
        .build();

    let null_checkbutton = gtk::CheckButton::builder().build();


    let devices_selection_expander_row_viewport_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    devices_selection_expander_row_viewport_box.add_css_class("boxed-list");
    devices_selection_expander_row_viewport_box.add_css_class("round-all-scroll");

    let devices_selection_expander_row_viewport =
    gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .has_frame(true)
        .child(&devices_selection_expander_row_viewport_box)
        .build();

    devices_selection_expander_row_viewport.add_css_class("round-all-scroll");

    devices_selection_expander_row.add_row(&devices_selection_expander_row_viewport);


    //
    language_changed_action.connect_activate(
        clone!(
            #[weak]
            automatic_partitioning_page,
            #[weak]
            devices_selection_expander_row,
            move |_, _| {
                automatic_partitioning_page.set_page_title(t!("auto_part_installer"));
                automatic_partitioning_page.set_page_subtitle(t!("choose_drive_auto"));
                automatic_partitioning_page.set_back_tooltip_label(t!("back"));
                automatic_partitioning_page.set_next_tooltip_label(t!("next"));
                //
                if devices_selection_expander_row.widget_name() == "status:disk=none," {
                    devices_selection_expander_row.set_title(&t!("no_drive_auto_selected"));
                }
            }
        )
    );
    //

    main_carousel.append(&automatic_partitioning_page);
}