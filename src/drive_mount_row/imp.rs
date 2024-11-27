use std::{cell::RefCell, rc::Rc, sync::OnceLock};

use adw::{prelude::*, subclass::prelude::*, *};
use glib::{clone, subclass::Signal, Properties};
use gtk::{glib};

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
    transient_for: Rc<RefCell<adw::ApplicationWindow>>,
    #[property(get, set)]
    sizegroup: RefCell<Option<gtk::SizeGroup>>,
    #[property(get, set)]
    langaction: RefCell<Option<gio::SimpleAction>>,
    #[property(get, set)]
    id: RefCell<i32>,
}
// ANCHOR_END: custom_button

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for DriveMountRow {
    const NAME: &'static str = "DriveMountRow";
    type Type = super::DriveMountRow;
    type ParentType = adw::ExpanderRow;
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
        self.parent_constructed();

        // Bind label to number
        // `SYNC_CREATE` ensures that the label will be immediately set
        let obj = self.obj();

        let is_selected = Rc::new(RefCell::new(false));

        let action_row_content_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(0)
            .vexpand(true)
            .hexpand(true)
            .build();

        let partition_button_row_adw_listbox = gtk::ListBox::builder()
            .hexpand(true)
            .vexpand(true)
            .selection_mode(gtk::SelectionMode::None)
            .margin_bottom(5)
            .margin_top(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        partition_button_row_adw_listbox.add_css_class("boxed-list");

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

        let partition_button_row = adw::ActionRow::builder()
            .title(t!("partition_button_row_title"))
            .subtitle(t!("partition_button_row_subtitle_none_selected"))
            .hexpand(true)
            .vexpand(true)
            .build();
        partition_button_row.add_prefix(&gtk::Image::from_icon_name("drive-harddisk-symbolic"));

        let partition_button = gtk::Button::builder()
            .valign(gtk::Align::Center)
            .hexpand(true)
            .vexpand(true)
            .child(&partition_button_row)
            .build();
        partition_button.add_css_class("flat");

        let mountpoint_entry_row = adw::EntryRow::builder()
            .title(t!("mountpoint_entry_row_title"))
            .hexpand(true)
            .vexpand(true)
            .margin_bottom(5)
            .margin_top(5)
            .margin_start(5)
            .margin_end(5)
            .build();

        let mountopts_entry_row = adw::EntryRow::builder()
            .title(t!("mountopts_entry_row_title"))
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

        obj.bind_property("deletable", &mountpoint_entry_row, "sensitive")
            .sync_create()
            .bidirectional()
            .build();

        obj.connect_partition_notify(clone!(
            #[strong]
            is_selected,
            #[strong]
            partition_button_row,
            move |obj| {
                let partition = obj.property::<String>("partition");
                let is_empty = partition.trim().is_empty();
                *is_selected.borrow_mut() = !is_empty;
                if is_empty {
                    partition_button_row
                        .set_subtitle(&t!("partition_button_row_subtitle_none_selected"));
                } else {
                    partition_button_row.set_subtitle(&partition);
                }
            }
        ));

        partition_row_delete_button.connect_clicked(clone!(
            #[weak]
            obj,
            move |_| {
                obj.emit_by_name::<()>("row-deleted", &[]);
            }
        ));

        //

        partition_button_row_adw_listbox.append(&partition_button);

        mountpoint_entry_row_adw_listbox.append(&mountpoint_entry_row);

        mountopts_entry_row_adw_listbox.append(&mountopts_entry_row);

        //

        action_row_content_box.append(&partition_button_row_adw_listbox);

        action_row_content_box.append(&mountpoint_entry_row_adw_listbox);

        action_row_content_box.append(&mountopts_entry_row_adw_listbox);

        action_row_content_box.append(&partition_row_delete_button);

        obj.connect_sizegroup_notify(clone!(
            #[weak]
            obj,
            #[weak]
            partition_button_row_adw_listbox,
            #[weak]
            mountpoint_entry_row_adw_listbox,
            #[weak]
            mountopts_entry_row_adw_listbox,
            move |_| {
                if let Some(t) = obj.sizegroup() {
                    t.add_widget(&partition_button_row_adw_listbox);
                    t.add_widget(&mountpoint_entry_row_adw_listbox);
                    t.add_widget(&mountopts_entry_row_adw_listbox);
                }
            }
        ));

        let partition_button_row_dialog_extra_child = adw::Bin::new();

        let partition_button_row_dialog = adw::AlertDialog::builder()
            .extra_child(&partition_button_row_dialog_extra_child)
            .width_request(400)
            .height_request(400)
            .title(t!("devices_selection_button_row_dialog_manual_title"))
            .body(t!("devices_selection_button_row_dialog_manual_body"))
            .build();

        partition_button_row_dialog.add_response(
            "devices_selection_button_row_dialog_ok",
            &t!("devices_selection_button_row_dialog_ok_label"),
        );

        partition_button.connect_clicked(clone!(
            #[strong]
            obj,
            #[strong]
            partition_button_row_dialog,
            move |_| {
                partition_button_row_dialog.present(Some(
                    &obj.property::<adw::ApplicationWindow>("transient_for"),
                ));
            }
        ));

        // Bind label to number
        // `SYNC_CREATE` ensures that the label will be immediately set
        let obj = self.obj();
        /*obj.bind_property("partition", &partition_button_row, "subtitle")
        .sync_create()
        .bidirectional()
        .build();*/

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
            #[weak]
            partition_button_row_dialog_extra_child,
            move |_| {
                partition_button_row_dialog_extra_child.set_child(Some(
                    &obj.property::<gtk::ScrolledWindow>("partitionscroll"),
                ));
            }
        ));

        obj.connect_langaction_notify(clone!(
            #[weak]
            obj,
            #[weak]
            partition_button_row,
            #[weak]
            mountpoint_entry_row,
            #[weak]
            mountopts_entry_row,
            #[weak]
            partition_button_row_dialog,
            #[strong]
            is_selected,
            move |_| {
                if let Some(t) = obj.langaction() {
                    t.connect_activate(clone!(
                        #[weak]
                        partition_button_row,
                        #[weak]
                        mountpoint_entry_row,
                        #[weak]
                        mountopts_entry_row,
                        #[weak]
                        partition_button_row_dialog,
                        #[strong]
                        is_selected,
                        move |_, _| {
                            if !*is_selected.borrow() {
                                partition_button_row.set_subtitle(&t!(
                                    "partition_button_row_subtitle_none_selected"
                                ));
                            }
                            mountpoint_entry_row.set_title(&t!("mountpoint_entry_row_title"));
                            mountopts_entry_row.set_title(&t!("mountopts_entry_row_title"));
                            //
                            partition_button_row_dialog
                                .set_title(&t!("devices_selection_button_row_dialog_manual_title"));
                            partition_button_row_dialog
                                .set_body(&t!("devices_selection_button_row_dialog_manual_body"));
                            partition_button_row_dialog.set_response_label(
                                "devices_selection_button_row_dialog_ok",
                                &t!("devices_selection_button_row_dialog_ok_label"),
                            );
                        }
                    ));
                }
            }
        ));

        obj.add_row(&action_row_content_box);
    }
}
// Trait shared by all widgets
impl WidgetImpl for DriveMountRow {}

impl ExpanderRowImpl for DriveMountRow {}

impl PreferencesRowImpl for DriveMountRow {}

impl ListBoxRowImpl for DriveMountRow {}
