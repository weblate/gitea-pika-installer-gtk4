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

pub fn install_page(content_stack: &gtk::Stack) {
    
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

    let install_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

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
        .subtitle("en_us")
        .build();
    install_confirm_detail_language.add_css_class("property");

    let install_confirm_detail_timezone = adw::ActionRow::builder()
        .title("Time zone:")
        .subtitle("Europe/Moscow")
        .build();
    install_confirm_detail_timezone.add_css_class("property");

    let install_confirm_detail_keyboard = adw::ActionRow::builder()
        .title("Keyboard layout:")
        .subtitle("us")
        .build();
    install_confirm_detail_keyboard.add_css_class("property");

    let install_confirm_detail_target = adw::ActionRow::builder()
        .title("Install Target:")
        .subtitle("/dev/sda1")
        .build();
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

    let the_terminal = vte::Terminal::builder()
        .vexpand(true)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .sensitive(false)
        .build();

    the_terminal.spawn_async(
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

    install_progress_box.append(&the_terminal);

    install_nested_stack.add_titled(&install_confirm_box, Some("confirm_page"), "confirm_page");
    install_nested_stack.add_titled(&install_progress_box, Some("progress_page"), "progress_page");

    // / Content stack appends
    //// Add the install_main_box as page: install_page, Give it nice title
    content_stack.add_titled(&install_main_box, Some("install_page"), "Welcome");

    install_confirm_button.connect_clicked(clone!(@weak install_nested_stack => move |_| install_nested_stack.set_visible_child_name("progress_page")));

    bottom_back_button.connect_clicked(clone!(@weak content_stack => move |_| {
        content_stack.set_visible_child_name("partitioning_page");
    }));
}