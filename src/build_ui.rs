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
use crate::save_window_size;
use crate::welcome_page;

use std::thread;
use std::time::*;
use fragile::*;


// build ui function linked to app startup above
pub fn build_ui(app: &adw::Application) {

    // setup glib
    gtk::glib::set_prgname(Some("PikaOS Installer"));
    glib::set_application_name("PikaOS Installer");
    let glib_settings = gio::Settings::new("com.github.pikaos-linux.pikainstallergtk4");


    // Widget Bank

    let gtk_loops = true;


    /// Create A box
    let _main_box = gtk::Box::builder()
        // that puts items vertically
        .orientation(Orientation::Vertical)
        .build();
    
    /// Add adwaita title box
    let window_title_bar = gtk::HeaderBar::builder()
        .show_title_buttons(true)
        .build();

    /// Add page Stack containing all primary contents
    let content_stack = gtk::Stack::builder()
        .hexpand(true)
        .vexpand(true)
        .build();
    
    /// Add a Visual Stack Switcher for content_stack
    let content_stack_switcher = gtk::StackSwitcher::builder()
        .stack(&content_stack)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    // create the bottom box for next and back buttons
    let bottom_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
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
    let bottom_next_button = gtk::Button::builder()
        .label("Next")
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .halign(gtk::Align::End)
        .hexpand(true)
        .build();
    
    // Start Applying css classes
    bottom_next_button.add_css_class("suggested-action");
    
    // / bottom_box appends
    //// Add the next and back buttons
    bottom_box.append(&bottom_back_button);
    bottom_box.append(&bottom_next_button);
    
    // / _main_box appends
    //// Add the a title bar to the _main_box
    _main_box.append(&window_title_bar); 
    //// Add the step indicator to _main_box
    _main_box.append(&content_stack_switcher);
    //// Add the stack pager containing all the steps to _main_box
    _main_box.append(&content_stack);
    //// Add the the next and back buttons box to _main_box
    _main_box.append(&bottom_box);

    // Add welcome_page.rs as a page for content_stack
    welcome_page(&content_stack);

    // create the main Application window
    let window = adw::ApplicationWindow::builder()
        // The text on the titlebar
        .title("PikaOS Installer")
        // link it to the application "app"
        .application(app)
        // Add the box called "_main_box" to it
        .content(&_main_box)
        // Application icon
        .icon_name("nautilus")
        // Get current size from glib
        .default_width(glib_settings.int("window-width"))
        .default_height(glib_settings.int("window-height"))
        // Minimum Size/Default
        .width_request(700)
        .height_request(500)
        // Hide window instead of destroy
        .hide_on_close(true)
        // Startup
        .startup_id("pika-installer-gtk4")
        // build the window
        .build();
    
    // glib maximization
    if glib_settings.boolean("is-maximized") == true {
        window.maximize()
    }
        
    // Connects the clicking of  "_click_me_button" to the external function "print_why" and idk why but everyone tells me to be "move |_| " before the external function
    /// and instead of () we put an aurgment for the target label with & before it so it's"
    /// print_why() -> print_why(&_warning_label)
    //_click_me_button.connect_clicked(move |_| print_why(&_warning_label));
        
    // Connect the hiding of window to the save_window_size function and window destruction
    window.connect_hide(clone!(@weak window => move |_| save_window_size(&window, &glib_settings)));
    window.connect_hide(clone!(@weak window => move |_| window.destroy()));
    
    let (sender, receiver) = MainContext::channel(Priority::default());
    window.connect_show(move |_| {
        let sender = sender.clone();
        // The long running operation runs now in a separate thread
        thread::spawn(move || {
           sender.send(false).expect("Could not send through channel");
        });
    });
        
    window.show();
    
    receiver.attach(
        None,
        clone!(@weak bottom_box => @default-return Continue(false),
                    move |state| {
                        bottom_box_loop(&bottom_box, state);
                        glib::ControlFlow::Continue
                    }
        ),

    );
    
}

fn bottom_box_loop(bottom_box: &gtk::Box, state: bool) {
        bottom_box.set_visible(state)
}