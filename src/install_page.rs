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
       .build();

   let install_confirm_button = gtk::Button::builder()
       .label("Confirm & Install PikaOS")
       .vexpand(true)
       .hexpand(true)
       .halign(gtk::Align::Center)
       .valign(gtk::Align::Start)
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
            10,
            None::<&gio::Cancellable>,
            move |result| {
                match result {
                    Ok(pid) => { eprintln!("could not spawn terminal:") }
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
}