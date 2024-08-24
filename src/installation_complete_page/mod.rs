use crate::{build_ui::PikaKeymap, config::LOG_FILE_PATH, installer_stack_page};
use adw::prelude::*;
use glib::{clone, closure_local};
use gnome_desktop::XkbInfoExt;
use gtk::{gio, glib, prelude::*};
use std::{cell::RefCell, fs, path::Path, process::Command, rc::Rc};

pub fn installation_complete_page(
    main_carousel: &adw::Carousel,
    window: &adw::ApplicationWindow,
    language_changed_action: &gio::SimpleAction,
    installation_log_status_loop_receiver: async_channel::Receiver<bool>,
) {
    let installation_complete_page = installer_stack_page::InstallerStackPage::new();
    
    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .homogeneous(true)
        .margin_bottom(15)
        .margin_top(15)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .vexpand(true)
        .hexpand(true)
        .build();

    let installation_complete_exit_button = gtk::Button::builder()
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .margin_start(5)
        .margin_end(5)
        .build();
    installation_complete_exit_button.add_css_class("destructive-action");
    installation_complete_exit_button.add_css_class("rounded-all-25-with-padding");

    let installation_complete_reboot_button = gtk::Button::builder()
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .margin_start(5)
        .margin_end(5)
        .build();
    installation_complete_reboot_button.add_css_class("suggested-action");
    installation_complete_reboot_button.add_css_class("rounded-all-25-with-padding");

    let installation_complete_view_logs_button = gtk::Button::builder()
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .margin_start(5)
        .margin_end(5)
        .build();
    installation_complete_view_logs_button.add_css_class("rounded-all-25-with-padding");

    content_box.append(&installation_complete_exit_button);
    content_box.append(&installation_complete_reboot_button);
    content_box.append(&installation_complete_view_logs_button);

    let installation_status_context = glib::MainContext::default();
    // The main loop executes the asynchronous block
    installation_status_context.spawn_local(
        clone!(
            #[strong]
            main_carousel,
            #[strong]
            installation_complete_page,
            #[strong]
            installation_log_status_loop_receiver,
            async move 
            {
                    while let Ok(state) = installation_log_status_loop_receiver.recv().await {
                        main_carousel.scroll_to(&installation_complete_page, true);
                        match state {
                            true => {
                                installation_complete_page.set_page_icon("emblem-ok-symbolic");
                                installation_complete_page.set_back_visible(false);
                                installation_complete_page.set_next_visible(false);
                                installation_complete_page.set_back_sensitive(false);
                                installation_complete_page.set_next_sensitive(false);
                                installation_complete_page.set_page_title(t!("installation_complete_page_title_success"));
                                installation_complete_page.set_page_subtitle(t!("installation_complete_page_subtitle_success"));
                            }
                            false => {
                                installation_complete_page.set_page_icon("dialog-error-symbolic");
                                installation_complete_page.set_back_visible(false);
                                installation_complete_page.set_next_visible(false);
                                installation_complete_page.set_back_sensitive(false);
                                installation_complete_page.set_next_sensitive(false);
                                installation_complete_page.set_page_title(t!("installation_complete_page_title_failed"));
                                installation_complete_page.set_page_subtitle(t!("installation_complete_page_subtitle_failed"));
                            }
                        }
                    }
            }
        )
    );

    //

    installation_complete_exit_button.connect_clicked(clone!(
        #[strong]
        window,
            move |_|
            {
                window.close()
            }
        )
    );
    
    installation_complete_reboot_button.connect_clicked(move |_| {
        Command::new("reboot")
            .spawn()
            .expect("reboot failed to start");
    });

    installation_complete_view_logs_button.connect_clicked(move |_| {
        Command::new("xdg-open")
            .arg(LOG_FILE_PATH)
            .spawn()
            .expect("xdg-open failed to start");
    });

    //

    installation_complete_page.set_child_widget(&content_box);

    //
    language_changed_action.connect_activate(clone!(
        #[weak]
        installation_complete_exit_button,
        #[weak]
        installation_complete_reboot_button,
        #[weak]
        installation_complete_view_logs_button,
        move |_, _| {
            installation_complete_exit_button.set_label(&t!("installation_complete_exit_button_label"));
            installation_complete_reboot_button.set_label(&t!("installation_complete_reboot_button_label"));
            installation_complete_view_logs_button.set_label(&t!("installation_complete_view_logs_button_label"));
        }
    ));
    //

    main_carousel.append(&installation_complete_page);
}
