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


use std::path::Path;

use crate::save_window_size::save_window_size;

use crate::welcome_page::welcome_page;

use crate::efi_error_page::efi_error_page;

use crate::language_page::language_page;

use crate::eula_page::eula_page;

use crate::timezone_page::timezone_page;

use crate::keyboard_page::keyboard_page;

use crate::partitioning_page::partitioning_page;

// build ui function linked to app startup above
pub fn build_ui(app: &adw::Application) {

    // setup glib
    gtk::glib::set_prgname(Some("PikaOS Installer"));
    glib::set_application_name("PikaOS Installer");
    let glib_settings = gio::Settings::new("com.github.pikaos-linux.pikainstallergtk4");


    // Widget Bank

    /// Create A box
    let _main_box = gtk::Box::builder()
        // that puts items vertically
        .orientation(Orientation::Vertical)
        .build();
    
    /// Add adwaita title box
    let window_title_bar = adw::HeaderBar::builder()
        .build();

    /// Add page Stack containing all primary contents
    let content_stack = gtk::Stack::builder()
        .hexpand(true)
        .vexpand(true)
        .transition_type(StackTransitionType::SlideLeftRight)
        .build();
    
    /// Add a Visual Stack Switcher for content_stack
    let content_stack_switcher = gtk::StackSwitcher::builder()
        .stack(&content_stack)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .sensitive(false)
        .build();
    
    // / _main_box appends
    //// Add the a title bar to the _main_box
    _main_box.append(&window_title_bar); 
    //// Add the step indicator to _main_box
    _main_box.append(&content_stack_switcher);
    //// Add the stack pager containing all the steps to _main_box
    _main_box.append(&content_stack);
    //// Add the the next and back buttons box to _main_box (moved)
    ///_main_box.append(&bottom_box);

    // create the main Application window
    let window = adw::ApplicationWindow::builder()
        // The text on the titlebar
        .title("PikaOS Installer")
        // link it to the application "app"
        .application(app)
        // Add the box called "_main_box" to it
        .content(&_main_box)
        // Application icon
        .icon_name("calamares")
        // Get current size from glib
        .default_width(glib_settings.int("window-width"))
        .default_height(glib_settings.int("window-height"))
        // Minimum Size/Default
        .width_request(700)
        .height_request(500)
        // Hide window instead of destroy
        .hide_on_close(true)
        //
        .deletable(false)
        // Startup
        .startup_id("pika-installer-gtk4")
        // build the window
        .build();
    
    // Add welcome_page.rs as a page for content_stack
    if Path::new("/sys/firmware/efi/efivars").exists() {
        welcome_page(&window, &content_stack);
    } else {
        efi_error_page(&window, &content_stack);
    }
    
    // bottom_box moved per page
    // if content_stack visible child becomes NOT content_stack, show the buttom box 
    //content_stack.connect_visible_child_notify(clone!(@weak bottom_box => move |content_stack| {
    //    let state = content_stack.visible_child_name().as_deref() != Some("welcome_page");
    //    bottom_box.set_visible(state);
    //  }));

    // Add language_page.rs as a page for content_stack
    language_page(&content_stack);

    // Add eula_page.rs as a page for content_stack
    eula_page(&content_stack);

    // Add timezone_page.rs as a page for content_stack
    timezone_page(&content_stack);

    // Add keyboard_page.rs as a page for content_stack
    keyboard_page(&content_stack);

    // Add install_page.rs as a page for content_stack
    let install_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let done_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // Add partitioning_page.rs as a page for content_stack
    partitioning_page(&done_main_box, &install_main_box,  &content_stack, &window);

    //// Add the install_main_box as page: install_page, Give it nice title
    content_stack.add_titled(&install_main_box, Some("install_page"), "Installation");

    // Add done_page.rs as a page for content_stack
    content_stack.add_titled(&done_main_box, Some("done_page"), "Done");

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
    // bottom_box moved per page
    //let content_stack_clone = content_stack.clone();
    //let content_stack_clone2 = content_stack.clone();
    //bottom_next_button.connect_clicked(move |_| content_stack_clone.set_visible_child(&content_stack_clone.visible_child().expect("null").next_sibling().unwrap()));
    //bottom_back_button.connect_clicked(move |_| content_stack_clone2.set_visible_child(&content_stack_clone2.visible_child().expect("null").prev_sibling().unwrap()));
    window.present();
}
