mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct InstallerStackPage(ObjectSubclass<imp::InstallerStackPage>)
    @extends adw::Bin, gtk::Widget;
}

impl InstallerStackPage {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
// ANCHOR_END: mod

impl Default for InstallerStackPage {
    fn default() -> Self {
        Self::new()
    }
}
