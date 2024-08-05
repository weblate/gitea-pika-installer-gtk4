use crate::installer_stack_page;
use crate::config;
use gtk::{prelude::*, glib as glib, Justification};
use adw::{prelude::*};
use glib::{clone, closure_local};
use std::{process::Command, env};
pub fn language_page(window: &adw::ApplicationWindow, main_carousel: &adw::Carousel) {
    let language_page = installer_stack_page::InstallerStackPage::new();
    language_page.set_page_title(t!("select_a_language"));
    language_page.set_page_subtitle(t!("please_select_locale"));
    language_page.set_page_icon("preferences-desktop-locale-symbolic");
    language_page.set_back_visible(true);
    language_page.set_next_visible(true);
    language_page.set_back_sensitive(true);
    language_page.set_next_sensitive(false);

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .margin_start(15)
        .margin_end(15)
        .margin_top(15)
        .margin_bottom(15)
        .build();

    let null_checkbutton = gtk::CheckButton::builder()
        .label(t!("no_locale_selected"))
        .build();

    let language_selection_row_viewport =
        gtk::ScrolledWindow::builder()
            .vexpand(true)
            .hexpand(true)
            .build();

    let language_selection_row_viewport_box = gtk::ListBox::builder().build();
    language_selection_row_viewport_box.add_css_class("boxed-list");

    language_selection_row_viewport
        .set_child(Some(&language_selection_row_viewport_box));

    let language_selection_row_viewport_listbox = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    language_selection_row_viewport_listbox.add_css_class("boxed-list");

    let language_search_bar = gtk::SearchEntry::builder()
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .placeholder_text(t!("search_for_language"))
        .search_delay(500)
        .build();

    let current_locale = match env::var_os("LANG") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$LANG is not set"),
    };

    let locale_list = ["ab_GE",
        "aa_DJ",
        "af_ZA",
        "ak_GH",
        "sq_AL",
        "am_ET",
        "ar_EG",
        "an_ES",
        "hy_AM",
        "as_IN",
        "ar_AE",
        "az_AZ",
        "bs_BA",
        "eu_ES",
        "be_BY",
        "bn_BD",
        "ar_BH",
        "bi_VU",
        "bs_BA",
        "br_FR",
        "bg_BG",
        "my_MM",
        "ca_ES",
        "de_CH",
        "ce_RU",
        "zh_CN",
        "cv_RU",
        "kw_GB",
        "es_CO",
        "es_CR",
        "hr_HR",
        "cs_CZ",
        "da_DK",
        "dv_MV",
        "nl_NL",
        "dz_BT",
        "en_US",
        "en_GB",
        "eo",
        "et_EE",
        "et_EE",
        "fo_FO",
        "hif_FJ",
        "fi_FI",
        "fr_FR",
        "ff_SN",
        "gl_ES",
        "ka_GE",
        "de_DE",
        "el_GR",
        "gu_IN",
        "ht_HT",
        "ha_NG",
        "he_IL",
        "hi_IN",
        "hu_HU",
        "ia_FR",
        "id_ID",
        "en_IE",
        "ga_IE",
        "ig_NG",
        "ik_CA",
        "is_IS",
        "it_IT",
        "iu_CA",
        "ja_JP",
        "kl_GL",
        "kn_IN",
        "ko_KR",
        "kk_KZ",
        "km_KH",
        "rw_RW",
        "ky_KG",
        "ky_KG",
        "ko_KR",
        "ku_TR",
        "lo_LA",
        "lb_LU",
        "lg_UG",
        "li_NL",
        "ln_CD",
        "lo_LA",
        "lt_LT",
        "fr_LU",
        "lv_LV",
        "gv_GB",
        "mk_MK",
        "mg_MG",
        "ms_MY",
        "ml_IN",
        "mt_MT",
        "mi_NZ",
        "mr_IN",
        "mn_MN",
        "ne_NP",
        "en_NG",
        "nb_NO",
        "nn_NO",
        "no_NO",
        "nr_ZA",
        "oc_FR",
        "es_CU",
        "om_ET",
        "or_IN",
        "os_RU",
        "pa_IN",
        "fa_IR",
        "pl_PL",
        "ps_AF",
        "pt_BR",
        "ro_RO",
        "ru_RU",
        "sa_IN",
        "sc_IT",
        "sd_IN",
        "se_NO",
        "sm_WS",
        "en_SG",
        "sr_RS",
        "gd_GB",
        "wo_SN",
        "si_LK",
        "sk_SK",
        "sl_SI",
        "so_SO",
        "st_ZA",
        "es_ES",
        "sw_KE",
        "ss_ZA",
        "sv_SE",
        "ta_IN",
        "te_IN",
        "tg_TJ",
        "th_TH",
        "ti_ER",
        "bo_CN",
        "tk_TM",
        "tl_PH",
        "tn_ZA",
        "to_TO",
        "tr_TR",
        "ts_ZA",
        "tt_RU",
        "zh_TW",
        "ug_CN",
        "uk_UA",
        "ur_PK",
        "ve_ZA",
        "vi_VN",
        "wa_BE",
        "cy_GB",
        "wo_SN",
        "fy_NL",
        "xh_ZA",
        "yi_US",
        "yo_NG",
        "zu_ZA",
        "zu_ZA",
        "pt_BR",
        "pt_PT",];

    let lang_data_buffer = gtk::TextBuffer::builder().build();

    for locale in locale_list.iter() {
        let locale = locale.to_string();
        let locale_name_cli =
            Command::new("/usr/lib/pika/pika-installer-gtk4/scripts/locale-name.py")
                .arg(locale.clone())
                .output()
                .expect("failed to execute process");
        let locale_name = String::from_utf8(locale_name_cli.stdout).unwrap();
        let locale_clone = locale.clone();
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
        language_selection_row_viewport_box.append(&locale_row);
        locale_checkbutton.connect_toggled(clone!(
            #[weak]
            locale_checkbutton,
            #[weak]
            lang_data_buffer,
            #[weak]
            language_page,
            move |_|
                {
                    if locale_checkbutton.is_active() == true {
                        language_page.set_next_sensitive(true);
                        lang_data_buffer.set_text(&locale);
                    }
                }
        ));
        if current_locale.contains(&(locale_clone))
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

    let lang_data_buffer_clone = lang_data_buffer.clone();

    language_search_bar.connect_search_changed(clone!(
        #[weak]
        language_search_bar,
        #[weak]
        language_selection_row_viewport_box,
        move |_|
        {
            let mut counter = language_selection_row_viewport_box.first_child();
            while let Some(row) = counter {
                if row.widget_name() == "AdwActionRow" {
                    if !language_search_bar.text().is_empty() {
                        if row.property::<String>("subtitle").to_lowercase().contains(&language_search_bar.text().to_string().to_lowercase()) || row.property::<String>("title").to_lowercase().contains(&language_search_bar.text().to_string().to_lowercase()) {
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
            move |language_page: installer_stack_page::InstallerStackPage|
            {
                    main_carousel.scroll_to(&main_carousel.nth_page(0), true)
            }
        )
    );

    language_page.connect_closure(
        "next-button-pressed",
        false,
        closure_local!(
            #[weak]
            main_carousel,
            move |language_page: installer_stack_page::InstallerStackPage|
            {
                    main_carousel.scroll_to(&main_carousel.nth_page(2), true)
            }
        )
    );

    main_carousel.append(&language_page);
}