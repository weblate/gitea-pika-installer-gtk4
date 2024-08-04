use std::path::Path;
use gtk::{prelude::*, glib as glib};
use vte::ffi::VTE_ALIGN_CENTER;
use crate::efi_error_page;
use crate::installer_stack_page;

pub fn build_ui(app: &adw::Application) {
    glib::set_prgname(Some("pikaos_installer"));
    glib::set_application_name(&t!("pikaos_installer"));

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
        .title(t!("pikaos_installer"))
        .application(app)
        .icon_name("calamares")
        .width_request(700)
        .height_request(500)
        .default_width(700)
        .default_height(500)
        .deletable(false)
        .content(&toolbarview)
        .startup_id("pika-installer-gtk4")
        .build();

    match Path::new("/sys/firmware/efi/efivars").exists() {
        true => {
            let page = installer_stack_page::InstallerStackPage::new();
            page.set_page_icon("pika-logo");
            page.set_page_title("Title");
            page.set_page_subtitle("Subtitle");
            let gbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
            gbox.append(&gtk::Image::builder().icon_name("pika-logo").build());
            page.set_child_widget(&gbox);
            carousel.append(&page);
        }
        _ => efi_error_page::efi_error_page(&window, &carousel)
    }
    //welcome_page(&window, &carousel),
    //language_page(&window, &carousel);

    window.present()
}