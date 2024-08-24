use crate::{build_ui::PikaKeymap, installer_stack_page};
use adw::prelude::*;
use glib::{clone, closure_local};
use gnome_desktop::XkbInfoExt;
use gtk::{gio, glib, prelude::*};
use std::{cell::RefCell, fs, path::Path, process::Command, rc::Rc};

pub fn installation_complete_page(
    main_carousel: &adw::Carousel,
    installation_log_status_loop_receiver: async_channel::Receiver<bool>,
) {
    let installation_complete_page = installer_stack_page::InstallerStackPage::new();
    
    let installation_status_context = glib::MainContext::default();
    // The main loop executes the asynchronous block
    installation_status_context.spawn_local(
        clone!(
            #[strong]
            main_carousel,
            #[strong]
            installation_complete_page,
            #[strong]
            installation_log_status_loop_receiver,
            async move 
            {
                    while let Ok(state) = installation_log_status_loop_receiver.recv().await {
                        main_carousel.scroll_to(&installation_complete_page, true);
                        match state {
                            true => {
                                installation_complete_page.set_page_icon("emblem-ok-symbolic");
                                installation_complete_page.set_back_visible(false);
                                installation_complete_page.set_next_visible(false);
                                installation_complete_page.set_back_sensitive(false);
                                installation_complete_page.set_next_sensitive(false);
                                installation_complete_page.set_page_title(t!("installation_complete_page_title_success"));
                                installation_complete_page.set_page_subtitle(t!("installation_complete_page_subtitle_success"));
                            }
                            false => {
                                installation_complete_page.set_page_icon("dialog-error-symbolic-ok-symbolic");
                                installation_complete_page.set_back_visible(false);
                                installation_complete_page.set_next_visible(false);
                                installation_complete_page.set_back_sensitive(false);
                                installation_complete_page.set_next_sensitive(false);
                                installation_complete_page.set_page_title(t!("installation_complete_page_title_success"));
                                installation_complete_page.set_page_subtitle(t!("installation_complete_page_subtitle_success"));
                            }
                        }
                    }
            }
        )
    );

    installation_complete_page.set_page_icon("keyboard-symbolic");
    installation_complete_page.set_back_visible(true);
    installation_complete_page.set_next_visible(true);
    installation_complete_page.set_back_sensitive(true);
    installation_complete_page.set_next_sensitive(false);

    main_carousel.append(&installation_complete_page);
}
