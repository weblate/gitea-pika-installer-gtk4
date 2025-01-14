use crate::{
    config::APP_ID, efi_error_page, eula_page, installation_complete_page,
    installation_progress_page, installation_summary_page, keyboard_page, language_page,
    partitioning_page, timezone_page, welcome_page,
};
use gtk::{gio, glib, prelude::*};
use std::{cell::RefCell, path::Path, rc::Rc};

// Custom Installer Data types

/// Locale Data types

#[derive(Default, Clone, Debug)]
pub struct PikaLocale {
    pub name: String,
    pub pretty_name: String,
}

/// Keyboard Data types

#[derive(Default, Clone, Debug)]
pub struct KBDMap {
    pub console: String,
    pub layout: String,
    pub variant: String,
}

#[derive(Default, Clone, Debug)]
pub struct PikaKeymap {
    pub kbdmap: KBDMap,
    pub pretty_name: String,
}

#[derive(Default, Clone, Debug)]
pub struct BlockDevice {
    pub block_name: String,
    pub block_model: String,
    pub block_size: f64,
    pub block_size_pretty: String,
}

/// Partitioning Data types

#[derive(Default, Clone, Debug)]
pub struct Partition {
    pub part_name: String,
    pub part_fs: String,
    pub part_uuid: String,
    pub has_encryption: bool,
    pub need_mapper: bool,
    pub part_size: f64,
    pub part_size_pretty: String,
}

#[derive(Default, Clone, Debug)]
pub struct FstabEntry {
    pub partition: Partition,
    pub mountpoint: String,
    pub mountopts: String,
    pub used_by: i32,
}

#[derive(Default, Clone, Debug)]
pub struct CrypttabEntry {
    pub partition: String,
    pub map: String,
    pub uuid: String,
    pub password: Option<String>,
}

pub struct SubvolDeclaration {
    pub part_name: Rc<std::cell::RefCell<String>>,
    pub made_by: Rc<std::cell::RefCell<i32>>,
}

pub fn build_ui(app: &adw::Application) {
    glib::set_prgname(Some(t!("application_name").to_string()));
    glib::set_application_name(&t!("application_name"));

    let (installation_log_loop_sender, installation_log_loop_receiver) = async_channel::unbounded();
    let installation_log_loop_sender: async_channel::Sender<String> =
        installation_log_loop_sender.clone();

    let (installation_log_status_loop_sender, installation_log_status_loop_receiver) =
        async_channel::unbounded();
    let installation_log_status_loop_sender: async_channel::Sender<bool> =
        installation_log_status_loop_sender.clone();

    let carousel = adw::Carousel::builder()
        .allow_long_swipes(false)
        .allow_mouse_drag(false)
        .allow_scroll_wheel(false)
        .interactive(false)
        .vexpand(true)
        .hexpand(true)
        .build();

    let carousel_indicator = adw::CarouselIndicatorDots::builder()
        .carousel(&carousel)
        .build();

    let window_headerbar = adw::HeaderBar::builder()
        .show_start_title_buttons(true)
        .title_widget(&carousel_indicator)
        .build();

    let toolbarview = adw::ToolbarView::builder()
        .top_bar_style(adw::ToolbarStyle::Flat)
        .content(&carousel)
        .build();

    toolbarview.add_top_bar(&window_headerbar);

    let window = adw::ApplicationWindow::builder()
        .title(t!("application_name"))
        .application(app)
        .icon_name("calamares")
        .width_request(700)
        .height_request(500)
        .default_width(1000)
        .default_height(700)
        .deletable(false)
        .content(&toolbarview)
        .startup_id(APP_ID)
        .build();

    match Path::new("/sys/firmware/efi/efivars").exists() {
        true => welcome_page::welcome_page(&window, &carousel),
        _ => efi_error_page::efi_error_page(&window, &carousel),
    }

    let language_selection_text_refcell: Rc<RefCell<PikaLocale>> = Rc::new(RefCell::default());
    let keymap_selection_text_refcell: Rc<RefCell<PikaKeymap>> = Rc::new(RefCell::default());
    let timezone_selection_text_refcell: Rc<RefCell<String>> = Rc::new(RefCell::default());
    let partition_method_type_refcell: Rc<RefCell<String>> = Rc::new(RefCell::default());
    let partition_method_automatic_target_refcell: Rc<RefCell<BlockDevice>> =
        Rc::new(RefCell::default());
    let partition_method_automatic_target_fs_refcell: Rc<RefCell<String>> =
        Rc::new(RefCell::default());
    let partition_method_automatic_luks_enabled_refcell: Rc<RefCell<bool>> =
        Rc::new(RefCell::new(false));
    let partition_method_automatic_luks_refcell: Rc<RefCell<String>> = Rc::new(RefCell::default());
    let partition_method_automatic_ratio_refcell: Rc<RefCell<f64>> = Rc::new(RefCell::new(0.0));
    let partition_method_automatic_seperation_refcell: Rc<RefCell<String>> =
        Rc::new(RefCell::default());
    let partition_method_manual_fstab_entry_array_refcell: Rc<RefCell<Vec<FstabEntry>>> =
        Rc::new(RefCell::new(Vec::new()));
    let partition_method_manual_luks_enabled_refcell: Rc<RefCell<bool>> =
        Rc::new(RefCell::new(false));
    let partition_method_manual_crypttab_entry_array_refcell: Rc<RefCell<Vec<CrypttabEntry>>> =
        Rc::new(RefCell::new(Vec::new()));

    let language_changed_action = gio::SimpleAction::new("lang-changed", None);
    let page_done_action = gio::SimpleAction::new("lang-changed", Some(glib::VariantTy::STRING));

    language_page::language_page(
        &carousel,
        &language_selection_text_refcell,
        &language_changed_action,
    );

    eula_page::eula_page(&carousel, &language_changed_action);

    keyboard_page::keyboard_page(
        &carousel,
        &keymap_selection_text_refcell,
        &language_changed_action,
    );

    timezone_page::timezone_page(
        &carousel,
        &timezone_selection_text_refcell,
        &language_changed_action,
    );

    partitioning_page::partitioning_page(
        &carousel,
        window.clone(),
        &partition_method_type_refcell,
        &partition_method_automatic_target_refcell,
        &partition_method_automatic_target_fs_refcell,
        &partition_method_automatic_luks_enabled_refcell,
        &partition_method_automatic_luks_refcell,
        &partition_method_automatic_ratio_refcell,
        &partition_method_automatic_seperation_refcell,
        &partition_method_manual_fstab_entry_array_refcell,
        &partition_method_manual_luks_enabled_refcell,
        &partition_method_manual_crypttab_entry_array_refcell,
        &language_changed_action,
        &page_done_action,
    );

    installation_summary_page::installation_summary_page(
        &carousel,
        &language_changed_action,
        &page_done_action,
        installation_log_loop_sender,
        installation_log_status_loop_sender,
        &language_selection_text_refcell,
        &keymap_selection_text_refcell,
        &timezone_selection_text_refcell,
        &partition_method_type_refcell,
        &partition_method_automatic_target_refcell,
        &partition_method_automatic_target_fs_refcell,
        &partition_method_automatic_luks_enabled_refcell,
        &partition_method_automatic_luks_refcell,
        &partition_method_automatic_ratio_refcell,
        &partition_method_automatic_seperation_refcell,
        &partition_method_manual_fstab_entry_array_refcell,
        &partition_method_manual_luks_enabled_refcell,
        &partition_method_manual_crypttab_entry_array_refcell,
    );

    installation_progress_page::installation_progress_page(
        &carousel,
        &language_changed_action,
        installation_log_loop_receiver,
    );

    installation_complete_page::installation_complete_page(
        &carousel,
        &window,
        &language_changed_action,
        installation_log_status_loop_receiver,
    );

    window.present()
}
