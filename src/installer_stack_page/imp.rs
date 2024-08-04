use std::{cell::RefCell, env, rc::Rc, sync::OnceLock};
use gtk::{prelude::*, subclass::prelude::*, glib as glib};
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

        let main_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        //

        let back_button = gtk::Button::builder()
            .icon_name("pan-start-symbolic")
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Start)
            .margin_start(5)
            .margin_end(5)
            .build();

        back_button.add_css_class("circular");

        back_button.connect_clicked(clone!(
            #[weak]
            obj,
            move |_| {
                obj.emit_by_name::<()>("back-button-pressed", &[]);
            }
        ));

        //

        let next_button = gtk::Button::builder()
            .icon_name("pan-end-symbolic")
            .valign(gtk::Align::Center)
            .halign(gtk::Align::End)
            .margin_start(5)
            .margin_end(5)
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

        //

        let content_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Center)
            .hexpand(true)
            .build();

        let info_status_page = adw::StatusPage::builder()
            .valign(gtk::Align::Start)
            .halign(gtk::Align::Center)
            .hexpand(true)
            .build();

        info_status_page.add_css_class("compact");

        obj.bind_property("page_icon", &info_status_page, "icon_name")
            .sync_create()
            .bidirectional()
            .build();

        obj.bind_property("page_title", &info_status_page, "title")
            .sync_create()
            .bidirectional()
            .build();

        obj.bind_property("page_subtitle", &info_status_page, "description")
            .sync_create()
            .bidirectional()
            .build();

        let child_bin = adw::Bin::builder()
            .build();

        content_box.append(&info_status_page);
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