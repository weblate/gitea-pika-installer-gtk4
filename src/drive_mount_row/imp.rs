use std::{cell::RefCell, env, rc::Rc, sync::OnceLock};

use adw::{prelude::*, subclass::prelude::*, *};
use gtk::{glib as glib, Orientation::Horizontal};
use glib::{clone, subclass::Signal, Properties};

use crate::partitioning_page::FstabEntry;

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
    sizegroup: Rc<RefCell<Option<gtk::SizeGroup>>>,
}
// ANCHOR_END: custom_button

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for DriveMountRow {
    const NAME: &'static str = "DriveMountRow";
    type Type = super::DriveMountRow;
    type ParentType = gtk::ListBoxRow;
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

        let action_row_content_box = gtk::Box::builder()
            .orientation(Horizontal)
            .spacing(0)
            .vexpand(true)
            .hexpand(true)
            .build();

        let partition_row_expander_adw_listbox = gtk::ListBox::builder()
            .hexpand(true)
            .vexpand(true)
            .selection_mode(gtk::SelectionMode::None)
            .margin_bottom(5)
            .margin_top(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        partition_row_expander_adw_listbox.add_css_class("boxed-list");

        let mountpoint_entry_row_adw_listbox = gtk::ListBox::builder()
            .hexpand(true)
            .vexpand(true)
            .valign(gtk::Align::Start)
            .selection_mode(gtk::SelectionMode::None)
            .margin_bottom(5)
            .margin_top(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        mountpoint_entry_row_adw_listbox.add_css_class("boxed-list");

        let mountopts_entry_row_adw_listbox = gtk::ListBox::builder()
            .hexpand(true)
            .vexpand(true)
            .valign(gtk::Align::Start)
            .selection_mode(gtk::SelectionMode::None)
            .margin_bottom(5)
            .margin_top(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        mountopts_entry_row_adw_listbox.add_css_class("boxed-list");

        let partition_row_expander = adw::ExpanderRow::builder()
            .subtitle(t!("subtitle_partition"))
            .build();

        let mountpoint_entry_row = adw::EntryRow::builder()
            .title(t!("title_mountpoint"))
            .hexpand(true)
            .vexpand(true)
            .margin_bottom(5)
            .margin_top(5)
            .margin_start(5)
            .margin_end(5)
            .build();

        let mountopts_entry_row = adw::EntryRow::builder()
            .title(t!("title_mountopts"))
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

        //

        partition_row_expander_adw_listbox.append(&partition_row_expander);

        mountpoint_entry_row_adw_listbox.append(&mountpoint_entry_row);

        mountopts_entry_row_adw_listbox.append(&mountopts_entry_row);

        //

        action_row_content_box.append(&partition_row_expander_adw_listbox);

        action_row_content_box.append(&mountpoint_entry_row_adw_listbox);

        action_row_content_box.append(&mountopts_entry_row_adw_listbox);

        action_row_content_box.append(&partition_row_delete_button);

        obj.connect_sizegroup_notify(clone!(
            #[weak]
            obj,
            #[weak]
            partition_row_expander_adw_listbox,
            #[weak]
            mountpoint_entry_row_adw_listbox,
            #[weak]
            mountopts_entry_row_adw_listbox,
            move |_|
                {
                    match obj.sizegroup() {
                        Some(t) => {
                            t.add_widget(&partition_row_expander_adw_listbox);
                            t.add_widget(&mountpoint_entry_row_adw_listbox);
                            t.add_widget(&mountopts_entry_row_adw_listbox);
                        }
                        None => {

                        }
                    }
                }
            )
        );

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

        obj.set_child(Some(&action_row_content_box));
    }
}
// Trait shared by all widgets
impl WidgetImpl for DriveMountRow {}

impl ListBoxRowImpl for DriveMountRow {}