mod imp;

use glib::Object;
use gtk::glib;

use crate::partitioning_page::FstabEntry;

glib::wrapper! {
    pub struct DriveMountRow(ObjectSubclass<imp::DriveMountRow>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl DriveMountRow {
    pub fn new() -> Self {
        Object::builder().build()
    }
    pub fn new_with_scroll(partitions_scroll: &gtk::ScrolledWindow) -> Self {
        Object::builder()
            .property("partitionscroll", partitions_scroll)
            .build()
    }
    pub fn get_fstab_entry(&self) -> FstabEntry {
        FstabEntry{
            partition: self.partition(),
            mountpoint: self.mountpoint(),
            mountopts: self.mountopts()
        }
    }
}
// ANCHOR_END: mod

impl Default for DriveMountRow {
    fn default() -> Self {
        Self::new()
    }
}
