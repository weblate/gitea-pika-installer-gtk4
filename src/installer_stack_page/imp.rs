use std::{cell::RefCell, env, rc::Rc, sync::OnceLock};
use gtk::{prelude::*, subclass::prelude::*, glib as glib, Justification};
use adw::{prelude::*, subclass::prelude::*};
use glib::{clone, subclass::prelude::*, subclass::Signal};

// ANCHOR: custom_button
// Object holding the state
#[derive(glib::Properties, Default)]
#[properties(wrapper_type = super::InstallerStackPage)]
pub struct InstallerStackPage {
    #[property(get, set)]
    page_icon: RefCell<String>,
    #[property(get, set)]
    page_title: RefCell<String>,
    #[property(get, set)]
    page_subtitle: RefCell<String>,
    #[property(get, set)]
    back_tooltip_label: RefCell<String>,
    #[property(get, set)]
    next_tooltip_label: RefCell<String>,
    #[property(get, set)]
    back_sensitive: RefCell<bool>,
    #[property(get, set)]
    next_sensitive: RefCell<bool>,
    #[property(get, set)]
    back_visible: RefCell<bool>,
    #[property(get, set)]
    next_visible: RefCell<bool>,
    #[property(get, set)]
    child_widget: Rc<RefCell<gtk::Box>>,
}
// ANCHOR_END: custom_button

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for InstallerStackPage {
    const NAME: &'static str = "InstallerStackPage";
    type Type = super::InstallerStackPage;
    type ParentType = adw::Bin;
}

// ANCHOR: object_impl
// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for InstallerStackPage {
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| vec![Signal::builder("next-button-pressed").build(), Signal::builder("back-button-pressed").build()])
    }
    fn constructed(&self) {
        self.parent_constructed();

        // Bind label to number
        // `SYNC_CREATE` ensures that the label will be immediately set
        let obj = self.obj();
        obj.set_hexpand(true);
        obj.set_vexpand(true);
        //

        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .build();
        //

        let back_button = gtk::Button::builder()
            .icon_name("pan-start-symbolic")
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Start)
            .sensitive(true)
            .visible(true)
            .margin_start(10)
            .margin_end(11)
            .build();

        back_button.add_css_class("circular");

        back_button.connect_clicked(clone!(
            #[weak]
            obj,
            move |_| {
                obj.emit_by_name::<()>("back-button-pressed", &[]);
            }
        ));

        obj.bind_property("back_tooltip_label", &back_button, "tooltip_text")
            .sync_create()
            .bidirectional()
            .build();

        obj.bind_property("back_sensitive", &back_button, "sensitive")
            .sync_create()
            .bidirectional()
            .build();

        obj.bind_property("back_visible", &back_button, "visible")
            .sync_create()
            .bidirectional()
            .build();

        //

        let next_button = gtk::Button::builder()
            .icon_name("pan-end-symbolic")
            .valign(gtk::Align::Center)
            .halign(gtk::Align::End)
            .sensitive(false)
            .visible(true)
            .margin_start(11)
            .margin_end(10)
            .build();

        next_button.add_css_class("circular");
        next_button.add_css_class("suggested-action");

        next_button.connect_clicked(clone!(
            #[weak]
            obj,
            move |_| {
                obj.emit_by_name::<()>("next-button-pressed", &[]);
            }
        ));

        obj.bind_property("next_tooltip_label", &next_button, "tooltip_text")
            .sync_create()
            .bidirectional()
            .build();

        obj.bind_property("next_sensitive", &next_button, "sensitive")
            .sync_create()
            .bidirectional()
            .build();

        obj.bind_property("next_visible", &next_button, "visible")
            .sync_create()
            .bidirectional()
            .build();

        //

        let content_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .build();

        //

        let installer_page_icon = gtk::Image::builder()
            .pixel_size(80)
            .valign(gtk::Align::Start)
            .halign(gtk::Align::Center)
            .hexpand(true)
            .build();

        let installer_page_title = gtk::Label::builder()
            .valign(gtk::Align::Start)
            .halign(gtk::Align::Center)
            .hexpand(true)
            .wrap(true)
            .justify(Justification::Center)
            .width_chars(20)
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();

        installer_page_title.add_css_class("title-1");

        let installer_page_subtitle = gtk::Label::builder()
            .valign(gtk::Align::Start)
            .halign(gtk::Align::Center)
            .hexpand(true)
            .justify(Justification::Center)
            .width_chars(20)
            .margin_top(5)
            .margin_start(5)
            .margin_end(5)
            .build();

        obj.bind_property("page_icon", &installer_page_icon, "icon_name")
            .sync_create()
            .bidirectional()
            .build();

        obj.bind_property("page_title", &installer_page_title, "label")
            .sync_create()
            .bidirectional()
            .build();

        obj.bind_property("page_subtitle", &installer_page_subtitle, "label")
            .sync_create()
            .bidirectional()
            .build();

        let child_bin = adw::Bin::builder()
            .vexpand(true)
            .hexpand(true)
            .margin_top(5)
            .margin_bottom(15)
            .build();

        content_box.append(&installer_page_icon);
        content_box.append(&installer_page_title);
        content_box.append(&installer_page_subtitle);
        content_box.append(&child_bin);

        obj.connect_child_widget_notify(clone!(
            #[weak]
            obj,
            #[weak]
            child_bin,
            move |_| {
                child_bin.set_child(Some(&obj.property::<gtk::Box>("child_widget")))
            }
        ));

        //

        main_box.append(&back_button);
        main_box.append(&content_box);
        main_box.append(&next_button);

        //

        obj.set_child(Some(&main_box));
    }
}

impl WidgetImpl for InstallerStackPage {}

impl BinImpl for InstallerStackPage {}