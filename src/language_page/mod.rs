// Use libraries
use std::env;
use adw::prelude::*;
use adw::*;
use glib::*;
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
/// Use all libadwaita libraries (libadwaita -> adw because cargo)
use gtk::*;

use std::process::Command;

use std::fs;
use std::path::Path;
use crate::eula_page::eula_page;
use crate::keyboard_page::keyboard_page;
use crate::partitioning_page::partitioning_page;
use crate::timezone_page::timezone_page;

pub fn language_page(content_stack: &gtk::Stack, window: &adw::ApplicationWindow) {
    // create the bottom box for next and back buttons
    let bottom_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .valign(gtk::Align::End)
        .vexpand(true)
        .build();

    // Next and back button
    let bottom_back_button = gtk::Button::builder()
        .label(t!("back"))
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .halign(gtk::Align::Start)
        .hexpand(true)
        .build();
    let bottom_next_button = gtk::Button::builder()
        .label(t!("next"))
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .halign(gtk::Align::End)
        .hexpand(true)
        .sensitive(false)
        .build();

    // Start Applying css classes
    bottom_next_button.add_css_class("suggested-action");

    // / bottom_box appends
    //// Add the next and back buttons
    bottom_box.append(&bottom_back_button);
    bottom_box.append(&bottom_next_button);

    // the header box for the language page
    let language_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // the header box for the language page
    let language_header_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    // the header text for the language page
    let language_header_text = gtk::Label::builder()
        .label(t!("select_a_language"))
        .halign(gtk::Align::End)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(5)
        .build();
    language_header_text.add_css_class("header_sized_text");

    // the header icon for the language icon
    let language_header_icon = gtk::Image::builder()
        .icon_name("locale")
        .halign(gtk::Align::Start)
        .hexpand(true)
        .pixel_size(78)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(0)
        .margin_end(15)
        .build();

    // make language selection box for choosing installation or live media
    let language_selection_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // / language_header_box appends
    //// Add the language page header text and icon
    language_header_box.append(&language_header_text);
    language_header_box.append(&language_header_icon);

    // / language_main_box appends
    //// Add the language header to language main box
    language_main_box.append(&language_header_box);
    //// Add the language selection/page content box to language main box
    language_main_box.append(&language_selection_box);

    // text above language selection box
    let language_selection_text = gtk::Label::builder()
        .label(t!("please_select_locale"))
        .halign(gtk::Align::Center)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    language_selection_text.add_css_class("medium_sized_text");

    let language_selection_expander_row = adw::ExpanderRow::builder()
        .title(t!("no_locale_selected"))
        .build();

    let null_checkbutton = gtk::CheckButton::builder()
        .label(t!("no_locale_selected"))
        .build();

    let language_selection_expander_row_viewport =
        gtk::ScrolledWindow::builder().height_request(420).build();

    let language_selection_expander_row_viewport_box = gtk::ListBox::builder().build();
    language_selection_expander_row_viewport_box.add_css_class("boxed-list");

    language_selection_expander_row_viewport
        .set_child(Some(&language_selection_expander_row_viewport_box));

    let language_selection_expander_row_viewport_listbox = gtk::ListBox::builder()
        .selection_mode(SelectionMode::None)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    language_selection_expander_row_viewport_listbox.add_css_class("boxed-list");
    language_selection_expander_row_viewport_listbox.append(&language_selection_expander_row);

    language_selection_expander_row.add_row(&language_selection_expander_row_viewport);

    let language_search_bar = gtk::SearchEntry::builder()
        .halign(gtk::Align::Center)
        .hexpand(true)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
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
            .valign(Align::Center)
            .can_focus(false)
            .build();
        let locale_row = adw::ActionRow::builder()
            .activatable_widget(&locale_checkbutton)
            .title(locale_name)
            .subtitle(locale.clone())
            .build();
        locale_row.add_prefix(&locale_checkbutton);
        locale_checkbutton.set_group(Some(&null_checkbutton));
        language_selection_expander_row_viewport_box.append(&locale_row);
        locale_checkbutton.connect_toggled(clone!(@weak locale_checkbutton, @weak language_selection_expander_row, @weak bottom_next_button, @weak lang_data_buffer => move |_| {
            if locale_checkbutton.is_active() == true {
                language_selection_expander_row.set_title(&locale_row.title());
                bottom_next_button.set_sensitive(true);
                lang_data_buffer.set_text(&locale);
            }
        }));
        if current_locale.contains(&(locale_clone))
            && current_locale != "C.UTF-8"
            && current_locale != "C"
            && current_locale != "C.utf8"
            && current_locale != "POSIX"
        {
            locale_checkbutton.set_active(true);
        }
    }

    // / language_selection_box appends
    //// add text and and entry to language page selections
    language_selection_box.append(&language_selection_text);
    language_selection_box.append(&language_search_bar);
    language_selection_box.append(&language_selection_expander_row_viewport_listbox);

    // / language_header_box appends
    //// Add the language page header text and icon
    language_header_box.append(&language_header_text);
    language_header_box.append(&language_header_icon);

    // / language_main_box appends
    //// Add the language header to language main box
    language_main_box.append(&language_header_box);
    //// Add the language selection/page content box to language main box
    language_main_box.append(&language_selection_box);

    language_main_box.append(&bottom_box);

    let lang_data_buffer_clone = lang_data_buffer.clone();

    language_search_bar.connect_search_changed(clone!(@weak language_search_bar, @weak language_selection_expander_row_viewport_box => move |_| {
        let mut counter = language_selection_expander_row_viewport_box.first_child();
        while let Some(row) = counter {
            if row.widget_name() == "AdwActionRow" {
                if !language_search_bar.text().is_empty() {
                    if row.property::<String>("subtitle").to_lowercase().contains(&language_search_bar.text().to_string().to_lowercase()) || row.property::<String>("title").to_lowercase().contains(&language_search_bar.text().to_string().to_lowercase()) {
                        language_selection_expander_row.set_expanded(true);
                        //row.grab_focus();
                        //row.add_css_class("highlight-widget");
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
    }));


    // / Content stack appends
    //// Add the language_main_box as page: language_page, Give it nice title
    content_stack.add_titled(
        &language_main_box,
        Some("language_page"),
        &t!("language"),
    );

    // the header box for the eula page
    let eula_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // / Content stack appends
    //// Add the eula_main_box as page: eula_page, Give it nice title
    content_stack.add_titled(&eula_main_box, Some("eula_page"), &t!("eula"));

    // the header box for the timezone page
    let timezone_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // / Content stack appends
    //// Add the keyboard_main_box as page: keyboard_page, Give it nice title
    content_stack.add_titled(
        &timezone_main_box,
        Some("timezone_page"),
        &t!("timezone"),
    );

    // the header box for the keyboard page
    let keyboard_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // / Content stack appends
    //// Add the keyboard_main_box as page: keyboard_page, Give it nice title
    content_stack.add_titled(
        &keyboard_main_box,
        Some("keyboard_page"),
        &t!("keyboard"),
    );

    // Add install_page.rs as a page for content_stack
    let install_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let done_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // the header box for the partitioning page
    let partitioning_main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    // / Content stack appends
    //// Add the partitioning_main_box as page: partitioning_page, Give it nice title
    content_stack.add_titled(
        &partitioning_main_box,
        Some("partitioning_page"),
        &t!("partitioning"),
    );

    //// Add the install_main_box as page: install_page, Give it nice title
    content_stack.add_titled(
        &install_main_box,
        Some("install_page"),
        &t!("installation"),
    );

    // Add done_page.rs as a page for content_stack
    content_stack.add_titled(&done_main_box, Some("done_page"), &t!("done"));

    bottom_next_button.connect_clicked(clone!(@weak content_stack, @weak window => move |_| {
        if Path::new("/tmp/pika-installer-gtk4-lang.txt").exists() {
            fs::remove_file("/tmp/pika-installer-gtk4-lang.txt").expect("Bad permissions on /tmp/pika-installer-gtk4-lang.txt");
        }
        fs::write("/tmp/pika-installer-gtk4-lang.txt", lang_data_buffer_clone.text(&lang_data_buffer_clone.bounds().0, &lang_data_buffer_clone.bounds().1, true).to_string()).expect("Unable to write file");
        Command::new("sudo")
        .arg("localectl")
        .arg("set-locale")
        .arg("LANG=".to_owned() + &lang_data_buffer_clone.text(&lang_data_buffer_clone.bounds().0, &lang_data_buffer_clone.bounds().1, true).to_string() + ".UTF-8")
        .spawn()
        .expect("locale failed to start");
        rust_i18n::set_locale(&lang_data_buffer_clone.text(&lang_data_buffer_clone.bounds().0, &lang_data_buffer_clone.bounds().1, true).to_string());
        // Add eula_page.rs as a page for content_stack
        while let Some(widget) = eula_main_box.last_child() {
                eula_main_box.remove(&widget);
        }
        eula_page(&content_stack, &eula_main_box);
        // Add timezone_page.rs as a page for content_stack
        while let Some(widget) = timezone_main_box.last_child() {
                timezone_main_box.remove(&widget);
        }
        timezone_page(&content_stack, &timezone_main_box);
        // Add keyboard_page.rs as a page for content_stack
        while let Some(widget) = keyboard_main_box.last_child() {
                keyboard_main_box.remove(&widget);
        }
        keyboard_page(&content_stack, &keyboard_main_box);
        // Add partitioning_page.rs as a page for content_stack
        while let Some(widget) = partitioning_main_box.last_child() {
                partitioning_main_box.remove(&widget);
        }
        partitioning_page(&partitioning_main_box, &done_main_box, &install_main_box, &content_stack, &window);
        //
        content_stack.set_visible_child_name("eula_page")
    }));
    bottom_back_button.connect_clicked(clone!(@weak content_stack => move |_| {
        content_stack.set_visible_child_name("welcome_page")
    }));
}
