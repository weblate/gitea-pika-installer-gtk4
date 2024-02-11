mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct DriveMountRow(ObjectSubclass<imp::DriveMountRow>)
        @extends adw::ActionRow, gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl DriveMountRow {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for DriveMountRow {
    fn default() -> Self {
        Self::new()
    }
}