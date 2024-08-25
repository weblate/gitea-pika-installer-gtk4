use crate::unix_socket_tools;
use adw::prelude::*;
use gtk::{gio, glib};
use glib::{clone, GString};
use std::thread;
use tokio::runtime::Runtime;


pub fn installation_progress_page(
    main_carousel: &adw::Carousel,
    language_changed_action: &gio::SimpleAction,
    installation_log_loop_receiver: async_channel::Receiver<String>,
) {
    let (socket_status_sender, socket_status_receiver) = async_channel::unbounded();
    let socket_status_sender: async_channel::Sender<String> = socket_status_sender.clone();

    thread::spawn(move || {
        Runtime::new().unwrap().block_on(unix_socket_tools::start_socket_server(
            socket_status_sender,
            "/tmp/pikainstall-status.sock",
        ));
    });
    
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
        .icon_name("pika-logo-text")
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
                if installation_progress_log_scroll.vadjustment().upper() - installation_progress_log_scroll.vadjustment().value() < (installation_progress_log_scroll.size(gtk::Orientation::Vertical) as f64 * 1.35 ) {
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

    let socket_status_context = glib::MainContext::default();
    // The main loop executes the asynchronous block
    socket_status_context.spawn_local(clone!(
        #[weak]
        installation_progress_bar,
        #[strong]
        socket_status_receiver,
        async move {
            while let Ok(state) = socket_status_receiver.recv().await {
                match state.trim() {
                    "PARTING" => {
                        installation_progress_bar.set_fraction(0.15);
                        installation_progress_bar.set_text(Some(&t!("installation_progress_bar_text_parting")));
                    }
                    "IMAGE" => {
                        installation_progress_bar.set_fraction(0.50);
                        installation_progress_bar.set_text(Some(&t!("installation_progress_bar_text_image")));
                    }
                    "FLAG" => {
                        installation_progress_bar.set_fraction(0.55);
                        installation_progress_bar.set_text(Some(&t!("installation_progress_bar_text_flag")));
                    }
                    "BIND" => {
                        installation_progress_bar.set_fraction(0.60);
                        installation_progress_bar.set_text(Some(&t!("installation_progress_bar_text_bind")));
                    }
                    "ARCH_COPY" => {
                        installation_progress_bar.set_fraction(0.65);
                        installation_progress_bar.set_text(Some(&t!("installation_progress_bar_text_arch_copy")));
                    }
                    "ENCRYPTION" => {
                        installation_progress_bar.set_fraction(0.70);
                        installation_progress_bar.set_text(Some(&t!("installation_progress_bar_text_encryption")));
                    }
                    "SWAP" => {
                        installation_progress_bar.set_fraction(0.75);
                        installation_progress_bar.set_text(Some(&t!("installation_progress_bar_text_swap")));
                    }
                    "GEN_FSTAB" => {
                        installation_progress_bar.set_fraction(0.80);
                        installation_progress_bar.set_text(Some(&t!("installation_progress_bar_text_gen_fstab")));
                    }
                    "LOCALE" => {
                        installation_progress_bar.set_fraction(0.85);
                        installation_progress_bar.set_text(Some(&t!("installation_progress_bar_text_locale")));
                    }
                    "BOOTLOADER" => {
                        installation_progress_bar.set_fraction(0.90);
                        installation_progress_bar.set_text(Some(&t!("installation_progress_bar_text_bootloader")));
                    }
                    "LIVE_REMOVE" => {
                        installation_progress_bar.set_fraction(0.95);
                        installation_progress_bar.set_text(Some(&t!("installation_progress_bar_text_live_remove")));
                    }
                    "BASIC_USER" => {
                        installation_progress_bar.set_fraction(0.98);
                        installation_progress_bar.set_text(Some(&t!("installation_progress_bar_text_basic_user")));
                    }
                    "UNBIND" => {
                        installation_progress_bar.set_fraction(0.99);
                        installation_progress_bar.set_text(Some(&t!("installation_progress_bar_text_unbind")));
                    }
                    _ => {}
                }
            }
        }
    ));

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