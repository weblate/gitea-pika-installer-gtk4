use adw::gio;
use crate::installer_stack_page;
use gtk::{prelude::*, glib as glib};
use glib::{clone, closure_local};

pub fn eula_page(
    main_carousel: &adw::Carousel,
    language_changed_action: &gio::SimpleAction
) {
    let eula_page = installer_stack_page::InstallerStackPage::new();
    eula_page.set_page_icon("error-correct-symbolic");
    eula_page.set_back_visible(true);
    eula_page.set_next_visible(true);
    eula_page.set_back_sensitive(true);
    eula_page.set_next_sensitive(false);

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();

    let eula_buffer = gtk::TextBuffer::builder()
        .build();

    let eula_selection_text_view = gtk::TextView::builder()
        .hexpand(true)
        .vexpand(true)
        .editable(false)
        .buffer(&eula_buffer)
        .build();

    let eula_selection_text_scroll = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .has_frame(true)
        .child(&eula_selection_text_view)
        .build();
    eula_selection_text_scroll.add_css_class("round-all-scroll");

    let eula_accept_checkbutton = gtk::CheckButton::builder()
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    eula_accept_checkbutton.connect_toggled(
        clone!(
            #[weak]
            eula_accept_checkbutton,
            #[weak]
            eula_page,
            move |_|
            {
                if eula_accept_checkbutton.is_active() == true {
                    eula_page.set_next_sensitive(true);
                } else {
                    eula_page.set_next_sensitive(false);
                }
            }
        ),
    );

    content_box.append(&eula_selection_text_scroll);
    content_box.append(&eula_accept_checkbutton);

    //
    language_changed_action.connect_activate(
        clone!(
            #[weak]
            eula_page,
            #[weak]
            eula_accept_checkbutton,
            #[strong]
            eula_buffer,
            move |_, _| {
                eula_page.set_page_title(t!("eula"));
                eula_page.set_page_subtitle(t!("pikaos_eula_agreement"));
                eula_page.set_back_tooltip_label(t!("back"));
                eula_page.set_next_tooltip_label(t!("next"));
                //
                eula_accept_checkbutton.set_label(Some(&t!("i_agree_eula")));
                //
                eula_buffer.set_text(&t!("eula_buffer"))
            }
        )
    );
    //

    eula_page.set_child_widget(&content_box);

    eula_page.connect_closure(
        "back-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            move |_language_page: installer_stack_page::InstallerStackPage|
            {
                    main_carousel.scroll_to(&main_carousel.nth_page(1), true)
            }
        )
    );

    eula_page.connect_closure(
        "next-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            move |_language_page: installer_stack_page::InstallerStackPage|
            {
                main_carousel.scroll_to(&main_carousel.nth_page(3), true)
            }
        )
    );

    main_carousel.append(&eula_page);
}