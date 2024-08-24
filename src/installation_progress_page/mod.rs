use crate::{
    build_ui::{BlockDevice, CrypttabEntry, FstabEntry, PikaKeymap, PikaLocale},
    config::{MINIMUM_BOOT_BYTE_SIZE, MINIMUM_EFI_BYTE_SIZE, DISTRO_ICON},
    installer_stack_page,
    installation_progress_page,
};
use adw::prelude::*;
use glib::{clone, closure_local, GString};
use gtk::{gio, glib};
use std::{cell::RefCell, fs, ops::Deref, path::Path, process::Command, rc::Rc};

/// DEBUG
use std::io::{self, Write};
use duct::cmd;
/// DEBUG END

pub fn installation_progress_page(
    main_carousel: &adw::Carousel,
    language_changed_action: &gio::SimpleAction,
    installation_log_loop_receiver: async_channel::Receiver<String>,
) {
    let installation_progress_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let installation_progress_log_stack = gtk::Stack::builder()
        .transition_type(gtk::StackTransitionType::SlideUpDown)
        .margin_start(15)
        .margin_end(15)
        .build();

    let installation_progress_log_terminal_buffer = gtk::TextBuffer::builder().build();

    let installation_progress_log_terminal = gtk::TextView::builder()
        .vexpand(true)
        .hexpand(true)
        .editable(false)
        .buffer(&installation_progress_log_terminal_buffer)
        .build();
    installation_progress_log_terminal.add_css_class("round-all-scroll");

    let installation_progress_log_scroll = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .has_frame(true)
        .child(&installation_progress_log_terminal)
        .build();
    installation_progress_log_scroll.add_css_class("round-all-scroll");

    let placeholder_icon = gtk::Image::builder()
        .icon_name(DISTRO_ICON)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .hexpand(true)
        .vexpand(true)
        .pixel_size(256)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build();

    let progress_bar_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_start(10)
        .margin_end(15)
        .build();

    let installation_progress_bar = gtk::ProgressBar::builder()
        .hexpand(true)
        .margin_end(5)
        .margin_top(5)
        .margin_bottom(5)
        .show_text(true)
        .build();

    let progress_log_button = gtk::Button::builder()
        .icon_name("terminal-symbolic")
        .halign(gtk::Align::End)
        .margin_start(5)
        .margin_top(5)
        .margin_bottom(5)
        .build();

    progress_bar_box.append(&installation_progress_bar);
    progress_bar_box.append(&progress_log_button);

    installation_progress_log_stack.add_titled(
        &placeholder_icon,
        Some("slideshow_page"),
        "slideshow_page",
    );
    installation_progress_log_stack.add_titled(
        &installation_progress_log_scroll,
        Some("terminal_log_page"),
        "terminal_log_page",
    );

    installation_progress_box.append(&installation_progress_log_stack);
    installation_progress_box.append(&progress_bar_box);

    //

    progress_log_button.connect_clicked(
        clone!(
            #[weak]
            installation_progress_log_stack,
            move |_| 
            {
                if installation_progress_log_stack.visible_child_name() == Some(GString::from_string_unchecked("slideshow_page".into())) {
                    installation_progress_log_stack.set_visible_child_name("terminal_log_page");
                } else {
                    installation_progress_log_stack.set_visible_child_name("slideshow_page");
                }
            }
        )
    );

    //

    installation_progress_log_terminal_buffer.connect_changed(clone!(
        #[weak]
        installation_progress_log_scroll,
        move |_|
            {
                if installation_progress_log_scroll.vadjustment().upper() - installation_progress_log_scroll.vadjustment().value() < (installation_progress_log_scroll.size(gtk::Orientation::Vertical) as f64 * 1.2 ) {
                    installation_progress_log_scroll.vadjustment().set_value(installation_progress_log_scroll.vadjustment().upper())
                }
            }
        )
    );

    //

    let installation_log_loop_context = glib::MainContext::default();
    // The main loop executes the asynchronous block
    installation_log_loop_context.spawn_local(
        clone!(
            #[weak]
            installation_progress_log_terminal_buffer,
            #[strong]
            installation_progress_log_terminal_buffer,
            async move 
            {
                    while let Ok(state) = installation_log_loop_receiver.recv().await {
                        installation_progress_log_terminal_buffer.insert(&mut installation_progress_log_terminal_buffer.end_iter(), &("\n".to_string() + &state))
                    }
            }
        )
    );

    //

    language_changed_action.connect_activate(clone!(
        #[weak]
        progress_log_button,
        move |_, _| {
            progress_log_button.set_tooltip_text(Some(&t!("progress_log_button_content_tooltip")));
        }
    ));

    main_carousel.append(&installation_progress_box);
}