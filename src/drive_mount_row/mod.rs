mod imp;

use glib::Object;
use gtk::glib;

use crate::{build_ui::FstabEntry, partitioning_page::create_parition_struct};

glib::wrapper! {
    pub struct DriveMountRow(ObjectSubclass<imp::DriveMountRow>)
        @extends gtk::Widget, adw::ExpanderRow,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl DriveMountRow {
    pub fn new() -> Self {
        Object::builder().build()
    }
    pub fn new_with_widgets(
        partitions_scroll: &gtk::ScrolledWindow,
        parent_window: &adw::ApplicationWindow,
    ) -> Self {
        Object::builder()
            .property("partitionscroll", partitions_scroll)
            .property("transient-for", parent_window)
            .build()
    }
    pub fn get_fstab_entry(&self) -> FstabEntry {
        FstabEntry {
            partition: create_parition_struct(&self.partition()),
            mountpoint: self.mountpoint(),
            mountopts: self.mountopts(),
            used_by: self.id(),
        }
    }
}
// ANCHOR_END: mod

impl Default for DriveMountRow {
    fn default() -> Self {
        Self::new()
    }
}
