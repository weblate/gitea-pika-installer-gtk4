mod imp;

use glib::Object;
use gtk::glib;
use gtk::ListBoxRow;

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends adw::ActionRow, gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl CustomButton {
    pub fn new() -> Self {
        Object::builder().build()
    }
}