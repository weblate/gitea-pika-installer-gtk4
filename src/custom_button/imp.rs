use gtk::glib;
use gtk::*;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use adw::subclass::prelude::*;
use adw::prelude::ActionRowExt;
use std::cell::{Cell, RefCell};
use glib::ObjectExt;
use gtk::Orientation::Horizontal;
use glib::GString;

// Object holding the state
#[derive(glib::Properties, Default)]
#[properties(wrapper_type = super::CustomButton)]
pub struct CustomButton {
    #[property(get, set)]
    filesystem: RefCell<String>,
    partition: RefCell<String>,
    mountpoint: RefCell<String>,
    partition_scroll: gtk::ScrolledWindow
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkAppCustomButton";
    type Type = super::CustomButton;
    type ParentType = adw::ActionRow;
}

// Trait shared by all GObjects
// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for CustomButton {
    fn constructed(&self) {
        self.parent_constructed();

        // Bind label to number
        // `SYNC_CREATE` ensures that the label will be immediately set
        let obj = self.obj();
        let action_row_content_box = gtk::Box::builder()
            .orientation(Horizontal)
            .spacing(0)
            .build();

        let partition_row_expander = adw::ExpanderRow::builder()
            .title("Partition")
            .build();

        action_row_content_box.append(&partition_row_expander);

        obj.add_prefix(&action_row_content_box)

        //obj.bind_property("number", obj.as_ref(), "label")
        //    .sync_create()
        //    .build();
    }
}

// Trait shared by all widgets
impl WidgetImpl for CustomButton {}

// Trait shared by all buttons
// Trait shared by all buttons

impl ListBoxRowImpl for CustomButton {}

impl PreferencesRowImpl for CustomButton {}

impl ActionRowImpl for CustomButton {
    //fn clicked(&self) {
    //    let incremented_number = self.obj().number() + 1;
    //    self.obj().set_number(incremented_number);
    //}
}