
// Use libraries
use adw::prelude::*;
use adw::*;
use glib::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::*;



pub fn eula_page(content_stack: &gtk::Stack,
                 eula_main_box: &gtk::Box,
) {

    // create the bottom box for next and back buttons
    let bottom_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .valign(gtk::Align::End)
        .vexpand(true)
        .build();

    // Next and back button
    let bottom_back_button = gtk::Button::builder()
        .label(t!("back"))
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .halign(gtk::Align::Start)
        .hexpand(true)
        .build();
    let bottom_next_button = gtk::Button::builder()
        .label(t!("next"))
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .halign(gtk::Align::End)
        .hexpand(true)
        .sensitive(false)
        .build();

    // Start Applying css classes
    bottom_next_button.add_css_class("suggested-action");

    // / bottom_box appends
    //// Add the next and back buttons
    bottom_box.append(&bottom_back_button);
    bottom_box.append(&bottom_next_button);

    // the header box for the eula page
    let eula_header_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    // the header text for the eula page
    let eula_header_text = gtk::Label::builder()
        .label(t!("pikaos_eula_agreement"))
        .halign(gtk::Align::End)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(5)
        .build();
    eula_header_text.add_css_class("header_sized_text");

    // the header icon for the eula icon
    let eula_header_icon = gtk::Image::builder()
        .icon_name("error-correct")
        .halign(gtk::Align::Start)
        .hexpand(true)
        .pixel_size(78)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(0)
        .margin_end(15)
        .build();

    // make eula selection box for choosing installation or live media
    let eula_selection_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // / eula_header_box appends
    //// Add the eula page header text and icon
    eula_header_box.append(&eula_header_text);
    eula_header_box.append(&eula_header_icon);

    // / eula_main_box appends
    //// Add the eula header to eula main box
    eula_main_box.append(&eula_header_box);
    //// Add the eula selection/page content box to eula main box
    eula_main_box.append(&eula_selection_box);

    // text above eula selection box
    let eula_selection_text = gtk::Label::builder()
        .label(t!("please_read_eula"))
        .halign(gtk::Align::Center)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    eula_selection_text.add_css_class("medium_sized_text");

    let eula_buffer = gtk::TextBuffer::builder()
        .text(t!("eula_buffer"))
        .build();

    let eula_selection_text_view = gtk::TextView::builder()
        .hexpand(true)
        .vexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .editable(false)
        .buffer(&eula_buffer)
        .build();

    let eula_selection_text_scroll = gtk::ScrolledWindow::builder()
        .height_request(350)
        .child(&eula_selection_text_view)
        .build();

    let eula_accept_checkbutton = gtk::CheckButton::builder()
        .label(t!("i_agree_eula"))
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    // / eula_selection_box appends
    //// add text and and entry to eula page selections
    eula_selection_box.append(&eula_selection_text);
    eula_selection_box.append(&eula_selection_text_scroll);
    eula_selection_box.append(&eula_accept_checkbutton);

    // / eula_header_box appends
    //// Add the eula page header text and icon
    eula_header_box.append(&eula_header_text);
    eula_header_box.append(&eula_header_icon);

    // / eula_main_box appends
    //// Add the eula header to eula main box
    eula_main_box.append(&eula_header_box);
    //// Add the eula selection/page content box to eula main box
    eula_main_box.append(&eula_selection_box);

    eula_main_box.append(&bottom_box);

    eula_accept_checkbutton.connect_toggled(
        clone!(@weak eula_accept_checkbutton, @weak bottom_next_button => move |_| {
            if eula_accept_checkbutton.is_active() == true {
                bottom_next_button.set_sensitive(true);
            } else {
                bottom_next_button.set_sensitive(false)
            }
        }),
    );

    bottom_next_button.connect_clicked(clone!(@weak content_stack => move |_| {
        content_stack.set_visible_child_name("timezone_page")
    }));
    bottom_back_button.connect_clicked(clone!(@weak content_stack => move |_| {
        content_stack.set_visible_child_name("language_page")
    }));
}
