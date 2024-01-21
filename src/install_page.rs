// Use libraries
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::prelude::*;
use gtk::*;
use adw::prelude::*;
use adw::*;
use glib::*;
use gdk::Display;
use gtk::subclass::layout_child;
use vte::prelude::*;
use vte::*;

use std::fs;
use std::path::Path;

pub fn install_page(install_main_box: &gtk::Box ,content_stack: &gtk::Stack) {
    
    // create the bottom box for next and back buttons
    let bottom_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .valign(gtk::Align::End)
        .vexpand(true)
        .build();
    
    // Next and back button
    let bottom_back_button = gtk::Button::builder()
        .label("Back")
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .halign(gtk::Align::Start)
        .hexpand(true)
        .build();
    
    // / bottom_box appends
    //// Add the next and back buttons
    bottom_box.append(&bottom_back_button);

    let install_nested_stack = gtk::Stack::builder()
        .transition_type(StackTransitionType::SlideLeftRight)
        .build();
   
   let install_confirm_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .build();

   // the header box for the install page
   let install_confirm_header_box = gtk::Box::builder()
       .orientation(Orientation::Horizontal)
       .build();

   // the header text for the install page
   let install_confirm_header_text = gtk::Label::builder()
       .label("Sit back, Relax, and watch the show.")
       .halign(gtk::Align::End)
       .hexpand(true)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(15)
       .margin_end(5)
       .build();
    install_confirm_header_text.add_css_class("header_sized_text");

   // the header icon for the install icon
   let install_confirm_header_icon = gtk::Spinner::builder()
        .halign(gtk::Align::Start)
        .hexpand(true)
        .margin_top(15)
       .margin_bottom(15)
       .margin_start(0)
       .margin_end(15)
       .build();
    install_confirm_header_icon.start();

   // make install selection box for choosing installation or live media 
   let install_confirm_selection_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .halign(gtk::Align::Fill)
       .valign(gtk::Align::Center)
       .vexpand(true)
       .hexpand(true)
       .build();

   let install_confirm_details_boxed_list = gtk::ListBox::builder()
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(256)
        .margin_end(256)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Center)
        .hexpand(true)
        .build();
    install_confirm_details_boxed_list.add_css_class("boxed-list");

    let install_confirm_detail_language = adw::ActionRow::builder()
        .title("Language:")
        .subtitle(fs::read_to_string("/tmp/pika-installer-gtk4-lang.txt").expect("Unable to read file"))
        .build();
    install_confirm_detail_language.add_css_class("property");

    let install_confirm_detail_timezone = adw::ActionRow::builder()
        .title("Time zone:")
        .subtitle(fs::read_to_string("/tmp/pika-installer-gtk4-timezone.txt").expect("Unable to read file"))
        .build();
    install_confirm_detail_timezone.add_css_class("property");

    let install_confirm_detail_keyboard = adw::ActionRow::builder()
        .title("Keyboard layout:")
        .subtitle(fs::read_to_string("/tmp/pika-installer-gtk4-keyboard.txt").expect("Unable to read file"))
        .build();
    install_confirm_detail_keyboard.add_css_class("property");

    let install_confirm_detail_target = adw::ActionRow::builder()
        .title("Install Target:")
        .build();

    if Path::new("/tmp/pika-installer-gtk4-target-manual.txt").exists() { 
        install_confirm_detail_target.set_subtitle(&fs::read_to_string("/tmp/pika-installer-gtk4-target-manual.txt").expect("Unable to read file"));
    } else {
        install_confirm_detail_target.set_subtitle(&fs::read_to_string("/tmp/pika-installer-gtk4-target-auto.txt").expect("Unable to read file"));
    }
    install_confirm_detail_target.add_css_class("property");
   
   let install_confirm_button = gtk::Button::builder()
       .label("Confirm & Install PikaOS")
       .halign(gtk::Align::Center)
       .valign(gtk::Align::Center)
       .build();
    install_confirm_button.add_css_class("destructive-action");
    
    // / install_confirm_header_box appends
    //// Add the install page header text and icon
    install_confirm_header_box.append(&install_confirm_header_text);
    install_confirm_header_box.append(&install_confirm_header_icon);
    
    // / install_confirm_box appends
    //// Add the install header to install main box
    install_confirm_box.append(&install_confirm_header_box);
    //// Add the install selection/page content box to install main box
    install_confirm_box.append(&install_confirm_selection_box);
    
    // Start Appending widgets to boxes

    // / install_confirm_selection_box appends
    //// add live and install media button to install page selections
    install_confirm_details_boxed_list.append(&install_confirm_detail_language);
    install_confirm_details_boxed_list.append(&install_confirm_detail_timezone);
    install_confirm_details_boxed_list.append(&install_confirm_detail_keyboard);
    install_confirm_details_boxed_list.append(&install_confirm_detail_target);
    //
    install_confirm_selection_box.append(&install_confirm_details_boxed_list);
    install_confirm_selection_box.append(&install_confirm_button);
    
    // / install_confirm_header_box appends
    //// Add the install page header text and icon
    install_confirm_header_box.append(&install_confirm_header_text);
    install_confirm_header_box.append(&install_confirm_header_icon);
    
    // / install_confirm_box appends
    //// Add the install header to install main box
    install_confirm_box.append(&install_confirm_header_box);
    //// Add the install selection/page content box to install main box
    install_confirm_box.append(&install_confirm_selection_box);

    install_main_box.append(&install_nested_stack);

    install_confirm_box.append(&bottom_box);

    ///
    
    let install_progress_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let install_progress_log_stack = gtk::Stack::builder()
        .transition_type(StackTransitionType::SlideUpDown)
        .build();

    let install_progress_log_terminal = vte::Terminal::builder()
        .vexpand(true)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .sensitive(false)
        .build();

    install_progress_log_terminal.spawn_async(
            PtyFlags::DEFAULT,
            Some(""),
            &["bash"],
            &[],
            SpawnFlags::DEFAULT,
            || {},
            -1,
            None::<&gio::Cancellable>,
            move |result| {
                match result {
                    Ok(_) => { eprintln!("could not spawn terminal")}
                    Err(err) => {
                        eprintln!("could not spawn terminal: {}", err);
                    }
                }
            },
        );    

    let placeholder_icon = gtk::Image::builder()
        .icon_name("debian-swirl")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .hexpand(true)
        .vexpand(true)
        .pixel_size(512)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    let progress_bar_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .margin_start(15)
        .margin_end(15)
        .build();

    let install_progress_bar = gtk::ProgressBar::builder()
        .hexpand(true)
        .margin_start(15)
        .margin_end(15)
        .margin_top(15)
        .margin_bottom(15)
        .build();

    let progress_log_button_content = adw::ButtonContent::builder()
        .label("View Logs")
        .icon_name("terminal")
        .build();

    let progress_log_button = gtk::Button::builder()
        .child(&progress_log_button_content)
        .margin_start(15)
        .margin_end(15)
        .margin_top(15)
        .margin_bottom(15)
        .build();

    progress_bar_box.append(&install_progress_bar);
    progress_bar_box.append(&progress_log_button);

    install_progress_log_stack.add_titled(&placeholder_icon, Some("slideshow_page"), "slideshow_page");
    install_progress_log_stack.add_titled(&install_progress_log_terminal, Some("terminal_log_page"), "terminal_log_page");

    install_progress_box.append(&install_progress_log_stack);
    install_progress_box.append(&progress_bar_box);

    install_nested_stack.add_titled(&install_confirm_box, Some("confirm_page"), "confirm_page");
    install_nested_stack.add_titled(&install_progress_box, Some("progress_page"), "progress_page");

    install_confirm_button.connect_clicked(clone!(@weak install_nested_stack => move |_| install_nested_stack.set_visible_child_name("progress_page")));

    progress_log_button.connect_clicked(clone!(@weak install_progress_log_stack => move |_| {
        if install_progress_log_stack.visible_child_name() == Some(GString::from_string_unchecked("slideshow_page".into())) {
            install_progress_log_stack.set_visible_child_name("terminal_log_page");
        } else {
            install_progress_log_stack.set_visible_child_name("slideshow_page");
        }
    }));
    
    bottom_back_button.connect_clicked(clone!(@weak content_stack, @weak install_main_box, @weak install_nested_stack => move |_| {
        content_stack.set_visible_child_name("partitioning_page");
        install_main_box.remove(&install_nested_stack)
    }));
}