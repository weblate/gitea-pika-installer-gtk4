use crate::{build_ui::PikaLocale, installer_stack_page};
use adw::prelude::*;
use glib::{clone, closure_local};
use gtk::{gio, glib};
use std::{cell::RefCell, env, process::Command, rc::Rc};

pub fn language_page(
    main_carousel: &adw::Carousel,
    lang_data_refcell: &Rc<RefCell<PikaLocale>>,
    language_changed_action: &gio::SimpleAction,
) {
    let language_page = installer_stack_page::InstallerStackPage::new();
    language_page.set_page_title(t!("language_page_title"));
    language_page.set_page_subtitle(t!("language_page_subtitle"));
    language_page.set_page_icon("preferences-desktop-locale-symbolic");
    language_page.set_back_tooltip_label(t!("back"));
    language_page.set_next_tooltip_label(t!("next"));
    language_page.set_back_visible(true);
    language_page.set_next_visible(true);
    language_page.set_back_sensitive(true);
    language_page.set_next_sensitive(false);

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();

    let null_checkbutton = gtk::CheckButton::builder().build();

    let language_selection_row_viewport_listbox = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    language_selection_row_viewport_listbox.add_css_class("boxed-list");
    language_selection_row_viewport_listbox.add_css_class("no-round-borders");

    let language_selection_row_viewport = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .has_frame(true)
        .overflow(gtk::Overflow::Hidden)
        .child(&language_selection_row_viewport_listbox)
        .build();

    language_selection_row_viewport.add_css_class("round-all-scroll-no-padding");

    let language_search_bar = gtk::SearchEntry::builder()
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .placeholder_text(t!("language_search_bar_placeholder_text"))
        .search_delay(500)
        .build();

    language_search_bar.add_css_class("rounded-all-25");

    let current_locale = match env::var_os("LANG") {
        Some(v) => v
            .into_string()
            .unwrap()
            .chars()
            .take_while(|&ch| ch != '.')
            .collect::<String>(),
        None => panic!("$LANG is not set"),
    };

    let locale_list = [
        "ab_GE", "aa_DJ", "af_ZA", "ak_GH", "sq_AL", "am_ET", "ar_EG", "an_ES", "hy_AM", "as_IN",
        "ar_AE", "az_AZ", "bs_BA", "eu_ES", "be_BY", "bn_BD", "ar_BH", "bi_VU", "bs_BA", "br_FR",
        "bg_BG", "my_MM", "ca_ES", "de_CH", "ce_RU", "zh_CN", "cv_RU", "kw_GB", "es_CO", "es_CR",
        "hr_HR", "cs_CZ", "da_DK", "dv_MV", "nl_NL", "dz_BT", "en_US", "en_GB", "eo", "et_EE",
        "et_EE", "fo_FO", "hif_FJ", "fi_FI", "fr_FR", "ff_SN", "gl_ES", "ka_GE", "de_DE", "el_GR",
        "gu_IN", "ht_HT", "ha_NG", "he_IL", "hi_IN", "hu_HU", "ia_FR", "id_ID", "en_IE", "ga_IE",
        "ig_NG", "ik_CA", "is_IS", "it_IT", "iu_CA", "ja_JP", "kl_GL", "kn_IN", "ko_KR", "kk_KZ",
        "km_KH", "rw_RW", "ky_KG", "ky_KG", "ko_KR", "ku_TR", "lo_LA", "lb_LU", "lg_UG", "li_NL",
        "ln_CD", "lo_LA", "lt_LT", "fr_LU", "lv_LV", "gv_GB", "mk_MK", "mg_MG", "ms_MY", "ml_IN",
        "mt_MT", "mi_NZ", "mr_IN", "mn_MN", "ne_NP", "en_NG", "nb_NO", "nn_NO", "no_NO", "nr_ZA",
        "oc_FR", "es_CU", "om_ET", "or_IN", "os_RU", "pa_IN", "fa_IR", "pl_PL", "ps_AF", "pt_BR",
        "ro_RO", "ru_RU", "sa_IN", "sc_IT", "sd_IN", "se_NO", "sm_WS", "en_SG", "sr_RS", "gd_GB",
        "wo_SN", "si_LK", "sk_SK", "sl_SI", "so_SO", "st_ZA", "es_ES", "sw_KE", "ss_ZA", "sv_SE",
        "ta_IN", "te_IN", "tg_TJ", "th_TH", "ti_ER", "bo_CN", "tk_TM", "tl_PH", "tn_ZA", "to_TO",
        "tr_TR", "ts_ZA", "tt_RU", "zh_TW", "ug_CN", "uk_UA", "ur_PK", "ve_ZA", "vi_VN", "wa_BE",
        "cy_GB", "wo_SN", "fy_NL", "xh_ZA", "yi_US", "yo_NG", "zu_ZA", "zu_ZA", "pt_BR", "pt_PT",
    ];

    let mut sorted_locale_vec = Vec::new();
    for locale in locale_list.iter() {
        sorted_locale_vec.push(PikaLocale {
            name: locale.to_string(),
            pretty_name: gnome_desktop::language_from_locale(locale, None)
                .unwrap_or(locale.to_string().into())
                .to_string(),
        })
    }
    sorted_locale_vec.sort_by_key(|k| k.pretty_name.clone());

    for pika_locale in sorted_locale_vec {
        let pika_locale_clone0 = pika_locale.clone();
        let locale = pika_locale.name;
        let locale_clone0 = locale.clone();
        let locale_name = pika_locale.pretty_name;
        let locale_checkbutton = gtk::CheckButton::builder()
            .valign(gtk::Align::Center)
            .can_focus(false)
            .build();
        let locale_row = adw::ActionRow::builder()
            .activatable_widget(&locale_checkbutton)
            .title(locale_name)
            .subtitle(locale.clone())
            .build();
        locale_row.add_prefix(&locale_checkbutton);
        locale_checkbutton.set_group(Some(&null_checkbutton));
        language_selection_row_viewport_listbox.append(&locale_row);
        locale_checkbutton.connect_toggled(clone!(
            #[weak]
            locale_checkbutton,
            #[strong]
            lang_data_refcell,
            #[strong]
            pika_locale_clone0,
            #[weak]
            language_page,
            move |_| {
                if locale_checkbutton.is_active() {
                    language_page.set_next_sensitive(true);
                    *lang_data_refcell.borrow_mut() = pika_locale_clone0.clone();
                }
            }
        ));
        if current_locale == locale_clone0
            && current_locale != "C.UTF-8"
            && current_locale != "C"
            && current_locale != "C.utf8"
            && current_locale != "POSIX"
        {
            locale_checkbutton.set_active(true);
        }
    }

    // / content_box appends
    //// add text and and entry to language page selections
    content_box.append(&language_search_bar);
    content_box.append(&language_selection_row_viewport);

    language_search_bar.connect_search_changed(clone!(
        #[weak]
        language_search_bar,
        #[weak]
        language_selection_row_viewport_listbox,
        move |_| {
            let mut counter = language_selection_row_viewport_listbox.first_child();
            while let Some(row) = counter {
                if row.widget_name() == "AdwActionRow" {
                    if !language_search_bar.text().is_empty() {
                        if row
                            .property::<String>("subtitle")
                            .to_lowercase()
                            .contains(&language_search_bar.text().to_string().to_lowercase())
                            || row
                                .property::<String>("title")
                                .to_lowercase()
                                .contains(&language_search_bar.text().to_string().to_lowercase())
                        {
                            row.set_property("visible", true);
                            language_search_bar.grab_focus();
                        } else {
                            row.set_property("visible", false);
                        }
                    } else {
                        row.set_property("visible", true);
                    }
                }
                counter = row.next_sibling();
            }
        }
    ));

    language_page.set_child_widget(&content_box);

    language_page.connect_closure(
        "back-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            move |_language_page: installer_stack_page::InstallerStackPage| {
                main_carousel.scroll_to(&main_carousel.nth_page(0), true)
            }
        ),
    );

    language_page.connect_closure(
        "next-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            #[strong]
            lang_data_refcell,
            #[strong]
            language_changed_action,
            move |_language_page: installer_stack_page::InstallerStackPage| {
                let locale = &lang_data_refcell.borrow();
                Command::new("sudo")
                    .arg("localectl")
                    .arg("set-locale")
                    .arg("LANG=".to_owned() + &locale.name + ".UTF-8")
                    .spawn()
                    .expect("locale failed to start");
                rust_i18n::set_locale(&locale.name);
                language_changed_action.activate(None);
                main_carousel.scroll_to(&main_carousel.nth_page(2), true)
            }
        ),
    );

    main_carousel.append(&language_page);
}
