use std::{cell::RefCell, env, rc::Rc, sync::OnceLock};

use adw::{prelude::*, subclass::prelude::*, *};
use gtk::{glib as glib, Orientation::Horizontal, SizeGroup};
use glib::{clone, subclass::Signal, Properties};

use crate::partitioning_page::FstabEntry;

struct DriveSizeGroup(gtk::SizeGroup);

impl Default for DriveSizeGroup {
    pub fn default(&self) -> Self {
        DriveSizeGroup::
    }
}

// ANCHOR: custom_button
// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::DriveMountRow)]
pub struct DriveMountRow {
    #[property(get, set)]
    partition: RefCell<String>,
    #[property(get, set)]
    mountpoint: RefCell<String>,
    #[property(get, set)]
    mountopts: RefCell<String>,
    #[property(get, set)]
    deletable: RefCell<bool>,
    #[property(get, set)]
    partitionscroll: Rc<RefCell<gtk::ScrolledWindow>>,
    #[property(get, set)]
    sizegroup: Rc<RefCell<DriveSizeGroup>>,
}
// ANCHOR_END: custom_button

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for DriveMountRow {
    const NAME: &'static str = "DriveMountRow";
    type Type = super::DriveMountRow;
    type ParentType = gtk::Box;
}

// ANCHOR: object_impl
// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for DriveMountRow {
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| vec![Signal::builder("row-deleted").build()])
    }
    fn constructed(&self) {
        let current_locale = match env::var_os("LANG") {
            Some(v) => v.into_string().unwrap(),
            None => panic!("$LANG is not set"),
        };
        rust_i18n::set_locale(current_locale.strip_suffix(".UTF-8").unwrap());

        self.parent_constructed();

        // Bind label to number
        // `SYNC_CREATE` ensures that the label will be immediately set
        let obj = self.obj();

        let partition_row_expander_adw_listbox = gtk::ListBox::builder()
            .hexpand(true)
            .vexpand(true)
            .margin_bottom(5)
            .margin_top(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        partition_row_expander_adw_listbox.add_css_class("boxed-list");

        let partition_row_expander = adw::ExpanderRow::builder()
            .subtitle(t!("subtitle_partition"))
            .build();

        let mountpoint_entry_row = gtk::Entry::builder()
            .placeholder_text(t!("title_mountpoint"))
            .hexpand(true)
            .vexpand(true)
            .margin_bottom(5)
            .margin_top(5)
            .margin_start(5)
            .margin_end(5)
            .build();

        let mountopts_entry_row = gtk::Entry::builder()
            .placeholder_text(t!("title_mountopts"))
            .hexpand(true)
            .vexpand(true)
            .margin_bottom(5)
            .margin_top(5)
            .margin_start(5)
            .margin_end(5)
            .build();

        let partition_row_delete_button = gtk::Button::builder()
            .vexpand(true)
            .margin_bottom(5)
            .margin_top(5)
            .margin_start(5)
            .margin_end(5)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .icon_name("user-trash")
            .build();

        obj.bind_property("deletable", &partition_row_delete_button, "visible")
            .sync_create()
            .bidirectional()
            .build();

        partition_row_delete_button.connect_clicked(clone!(
            #[weak]
            obj,
            move |_|
                {
                    obj.emit_by_name::<()>("row-deleted", &[]);
                }
            )
        );

        partition_row_expander_adw_listbox.append(&partition_row_expander);
        obj.append(&partition_row_expander_adw_listbox);

        obj.append(&mountpoint_entry_row);

        obj.append(&mountopts_entry_row);

        obj.append(&partition_row_delete_button);

        // Bind label to number
        // `SYNC_CREATE` ensures that the label will be immediately set
        let obj = self.obj();
        obj.bind_property("partition", &partition_row_expander, "title")
            .sync_create()
            .bidirectional()
            .build();

        obj.bind_property("mountpoint", &mountpoint_entry_row, "text")
            .sync_create()
            .bidirectional()
            .build();

        obj.bind_property("mountopts", &mountopts_entry_row, "text")
            .sync_create()
            .bidirectional()
            .build();

        obj.connect_partitionscroll_notify(clone!(
            #[weak]
            obj,
            move |_|
                {
                    partition_row_expander.add_row(&obj.property::<gtk::ScrolledWindow>("partitionscroll"));
                }
            )
        );
    }
}
// Trait shared by all widgets
impl WidgetImpl for DriveMountRow {}

impl BoxImpl for DriveMountRow {}