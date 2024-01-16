use gtk::pango::Language;
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

pub fn language_page(content_stack: &gtk::Stack) {
   // the header box for the language page
   let language_main_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .build();

   // the header box for the language page
   let language_header_box = gtk::Box::builder()
       .orientation(Orientation::Horizontal)
       .build();

   // the header text for the language page
   let language_header_text = gtk::Label::builder()
       .label("Select a language")
       .halign(gtk::Align::End)
       .hexpand(true)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(15)
       .margin_end(5)
       .build();
    language_header_text.add_css_class("header_sized_text");

   // the header icon for the language icon
   let language_header_icon = gtk::Image::builder()
       .icon_name("locale")
       .halign(gtk::Align::Start)
       .hexpand(true)
       .pixel_size(78)
       .margin_top(15)
       .margin_bottom(15)
       .margin_start(0)
       .margin_end(15)
       .build();

   // make language selection box for choosing installation or live media 
   let language_selection_box = gtk::Box::builder()
       .orientation(Orientation::Vertical)
       .build();

    // / language_header_box appends
    //// Add the language page header text and icon
    language_header_box.append(&language_header_text);
    language_header_box.append(&language_header_icon);
    
    // / language_main_box appends
    //// Add the language header to language main box
    language_main_box.append(&language_header_box);
    //// Add the language selection/page content box to language main box
    language_main_box.append(&language_selection_box);

    // text above language selection box
    let language_selection_text = gtk::Label::builder()
        .label("Please select a locale for the system to use")
        .halign(gtk::Align::Center)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    language_selection_text.add_css_class("medium_sized_text");

    let language_selection_expander_row = adw::ExpanderRow::builder()
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .title("Locale")
        .build();

    let locales = ["en_US", "russian", "en_GB"];
    let null_checkbutton = gtk::CheckButton::builder()
        .label("No locale selected")
        .build();
    language_selection_expander_row.add_row(&null_checkbutton);

    for locale in locales {
        let locale_label = gtk::CheckButton::builder()
            .label(locale)
            .build();
        locale_label.set_group(Some(&null_checkbutton));
        language_selection_expander_row.add_row(&locale_label);
    }




    // / language_selection_box appends
    //// add text and and entry to language page selections
    language_selection_box.append(&language_selection_text);
    language_selection_box.append(&language_selection_expander_row);
    
    // / language_header_box appends
    //// Add the language page header text and icon
    language_header_box.append(&language_header_text);
    language_header_box.append(&language_header_icon);
    
    // / language_main_box appends
    //// Add the language header to language main box
    language_main_box.append(&language_header_box);
    //// Add the language selection/page content box to language main box
    language_main_box.append(&language_selection_box);
    
    // / Content stack appends
    //// Add the language_main_box as page: language_page, Give it nice title
    content_stack.add_titled(&language_main_box, Some("language_page"), "Language");
}