use crate::build_ui::{BlockDevice, CrypttabEntry, FstabEntry, PikaKeymap, PikaLocale};
use std::{cell::RefCell, rc::Rc};

mod auto_basic;
mod auto_btrfs;
mod auto_ext4;
mod auto_xfs;
mod manual_basic;

pub const STANDARD_INSTALLATION_PROG: &str = r###"#! /bin/bash
set -e

SOCKET_PATH="/tmp/pikainstall-status.sock"

rm -rf /tmp/pika-installer-gtk4-swaplist
rm -rf /tmp/PIKA_CRYPT

PIKA_INSTALL_CHROOT_PATH='{CHROOT_PATH}'
PIKA_INSTALL_LOCALE='{LOCALE}.UTF-8'
PIKA_INSTALL_KEYMAP_BASE='{KEYMAP_BASE}'
PIKA_INSTALL_KEYMAP_VARIANT='{KEYMAP_VARIANT}'
PIKA_INSTALL_TIMEZONE='{TIMEZONE}'
"###;

pub fn create_installation_script(
    language_selection_text_refcell: &Rc<RefCell<PikaLocale>>,
    keymap_selection_text_refcell: &Rc<RefCell<PikaKeymap>>,
    timezone_selection_text_refcell: &Rc<RefCell<String>>,
    partition_method_type_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_target_refcell: &Rc<RefCell<BlockDevice>>,
    partition_method_automatic_target_fs_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_luks_enabled_refcell: &Rc<RefCell<bool>>,
    partition_method_automatic_luks_refcell: &Rc<RefCell<String>>,
    partition_method_automatic_ratio_refcell: &Rc<RefCell<f64>>,
    partition_method_automatic_seperation_refcell: &Rc<RefCell<String>>,
    partition_method_manual_fstab_entry_array_refcell: &Rc<RefCell<Vec<FstabEntry>>>,
    partition_method_manual_luks_enabled_refcell: &Rc<RefCell<bool>>,
    partition_method_manual_crypttab_entry_array_refcell: &Rc<RefCell<Vec<CrypttabEntry>>>,
) -> String {
    let mut final_script = String::new();
    
    let variant = match keymap_selection_text_refcell.borrow().clone().variant {
        Some(t) => {
            t.replace("'", r###"'"'"'"###)
        }
        None => "".to_string(),
    };

    let standard_installation_format = strfmt::strfmt(
        STANDARD_INSTALLATION_PROG,
        &std::collections::HashMap::from([
            ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
            (
                "LOCALE".to_string(),
                language_selection_text_refcell.borrow().name.replace("'", r###"'"'"'"###).as_str(),
            ),
            (
                "KEYMAP_BASE".to_string(),
                keymap_selection_text_refcell.borrow().name.replace("'", r###"'"'"'"###).as_str(),
            ),
            (
                "KEYMAP_VARIANT".to_string(),
                variant.as_str()
            ),
            (
                "TIMEZONE".to_string(),
                timezone_selection_text_refcell.borrow().replace("'", r###"'"'"'"###).as_str(),
            ),
        ]),
    )
    .unwrap();

    final_script.push_str(&standard_installation_format);

    match partition_method_type_refcell.borrow().as_str() {
        "automatic" => {
            let is_encrypted = *partition_method_automatic_luks_enabled_refcell.borrow();
            //
            let automatic_standard_installation_format = strfmt::strfmt(
                auto_basic::AUTOMATIC_STANDARD_INSTALLATION_PROG,
                &std::collections::HashMap::from([
                    (
                        "AUTO_INSTALL_TARGET_DISK".to_string(),
                        partition_method_automatic_target_refcell.borrow().block_name.replace("'", r###"'"'"'"###).as_str(),
                    ),
                ]),
            )
            .unwrap();

            final_script.push_str(&automatic_standard_installation_format);

            //
            match &*partition_method_automatic_target_fs_refcell.borrow().as_str().to_lowercase() {
                "btrfs" => {
                    match partition_method_automatic_seperation_refcell.borrow().as_str() {
                        "subvol" => {
                            if is_encrypted {
                                final_script.push_str(&strfmt::strfmt(
                                    auto_btrfs::AUTOMATIC_HOME_SUBVOL_BTRFS_LOCKED_INSTALLATION_PROG,
                                    &std::collections::HashMap::from([
                                        ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
                                        (
                                            "AUTO_LUKS_PASSWORD".to_string(),
                                            partition_method_automatic_luks_refcell.borrow().replace("'", r###"'"'"'"###).as_str(),
                                        )
                                    ]),
                                )
                                .unwrap());
                            } else {
                                final_script.push_str(&strfmt::strfmt(
                                    auto_btrfs::AUTOMATIC_HOME_SUBVOL_BTRFS_OPEN_INSTALLATION_PROG,
                                    &std::collections::HashMap::from([
                                        ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
                                    ]),
                                )
                                .unwrap());
                            }
                        }
                        "partition" => {
                            if is_encrypted {
                                final_script.push_str(&strfmt::strfmt(
                                    auto_btrfs::AUTOMATIC_HOME_PART_BTRFS_LOCKED_INSTALLATION_PROG,
                                    &std::collections::HashMap::from([
                                        ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
                                        (
                                            "AUTO_LUKS_PASSWORD".to_string(),
                                            partition_method_automatic_luks_refcell.borrow().replace("'", r###"'"'"'"###).as_str(),
                                        ),
                                        (
                                            "ROOT_PART_SIZE_AS_I64_MIB".to_string(),
                                            (partition_method_automatic_ratio_refcell.borrow().round() as i64 / 1048576).to_string().replace("'", r###"'"'"'"###).as_str(),
                                        )
                                    ]),
                                )
                                .unwrap());
                            } else {
                                final_script.push_str(&strfmt::strfmt(
                                    auto_btrfs::AUTOMATIC_HOME_PART_BTRFS_OPEN_INSTALLATION_PROG,
                                    &std::collections::HashMap::from([
                                        ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
                                        (
                                            "ROOT_PART_SIZE_AS_I64_MIB".to_string(),
                                            (partition_method_automatic_ratio_refcell.borrow().round() as i64 / 1048576).to_string().replace("'", r###"'"'"'"###).as_str(),
                                        )
                                    ]),
                                )
                                .unwrap());
                            }
                        }
                        "none" => {
                            if is_encrypted {
                                final_script.push_str(&strfmt::strfmt(
                                    auto_btrfs::AUTOMATIC_HOME_NONE_BTRFS_LOCKED_INSTALLATION_PROG,
                                    &std::collections::HashMap::from([
                                        ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
                                        (
                                            "AUTO_LUKS_PASSWORD".to_string(),
                                            partition_method_automatic_luks_refcell.borrow().replace("'", r###"'"'"'"###).as_str(),
                                        )
                                    ]),
                                )
                                .unwrap());
                            } else {
                                final_script.push_str(&strfmt::strfmt(
                                    auto_btrfs::AUTOMATIC_HOME_NONE_BTRFS_OPEN_INSTALLATION_PROG,
                                    &std::collections::HashMap::from([
                                        ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
                                    ]),
                                )
                                .unwrap());
                            }
                        }
                        _ => panic!()
                    }
                }
                "ext4" => {
                    match partition_method_automatic_seperation_refcell.borrow().as_str() {
                        "partition" => {
                            if is_encrypted {
                                final_script.push_str(&strfmt::strfmt(
                                    auto_ext4::AUTOMATIC_HOME_PART_EXT4_LOCKED_INSTALLATION_PROG,
                                    &std::collections::HashMap::from([
                                        ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
                                        (
                                            "AUTO_LUKS_PASSWORD".to_string(),
                                            partition_method_automatic_luks_refcell.borrow().replace("'", r###"'"'"'"###).as_str(),
                                        ),
                                        (
                                            "ROOT_PART_SIZE_AS_I64_MIB".to_string(),
                                            (partition_method_automatic_ratio_refcell.borrow().round() as i64 / 1048576).to_string().replace("'", r###"'"'"'"###).as_str(),
                                        )
                                    ]),
                                )
                                .unwrap());
                            } else {
                                final_script.push_str(&strfmt::strfmt(
                                    auto_ext4::AUTOMATIC_HOME_PART_EXT4_OPEN_INSTALLATION_PROG,
                                    &std::collections::HashMap::from([
                                        ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
                                        (
                                            "ROOT_PART_SIZE_AS_I64_MIB".to_string(),
                                            (partition_method_automatic_ratio_refcell.borrow().round() as i64 / 1048576).to_string().replace("'", r###"'"'"'"###).as_str(),
                                        )
                                    ]),
                                )
                                .unwrap());
                            }
                        }
                        "none" => {
                            if is_encrypted {
                                final_script.push_str(&strfmt::strfmt(
                                    auto_ext4::AUTOMATIC_HOME_NONE_EXT4_LOCKED_INSTALLATION_PROG,
                                    &std::collections::HashMap::from([
                                        ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
                                        (
                                            "AUTO_LUKS_PASSWORD".to_string(),
                                            partition_method_automatic_luks_refcell.borrow().replace("'", r###"'"'"'"###).as_str(),
                                        )
                                    ]),
                                )
                                .unwrap());
                            } else {
                                final_script.push_str(&strfmt::strfmt(
                                    auto_ext4::AUTOMATIC_HOME_NONE_EXT4_OPEN_INSTALLATION_PROG,
                                    &std::collections::HashMap::from([
                                        ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
                                    ]),
                                )
                                .unwrap());
                            }
                        }
                        _ => panic!()
                    }
                }
                "xfs" => {
                    match partition_method_automatic_seperation_refcell.borrow().as_str() {
                        "partition" => {
                            if is_encrypted {
                                final_script.push_str(&strfmt::strfmt(
                                    auto_xfs::AUTOMATIC_HOME_PART_XFS_LOCKED_INSTALLATION_PROG,
                                    &std::collections::HashMap::from([
                                        ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
                                        (
                                            "AUTO_LUKS_PASSWORD".to_string(),
                                            partition_method_automatic_luks_refcell.borrow().replace("'", r###"'"'"'"###).as_str(),
                                        ),
                                        (
                                            "ROOT_PART_SIZE_AS_I64_MIB".to_string(),
                                            (partition_method_automatic_ratio_refcell.borrow().round() as i64 / 1048576).to_string().replace("'", r###"'"'"'"###).as_str(),
                                        )
                                    ]),
                                )
                                .unwrap());
                            } else {
                                final_script.push_str(&strfmt::strfmt(
                                    auto_xfs::AUTOMATIC_HOME_PART_XFS_OPEN_INSTALLATION_PROG,
                                    &std::collections::HashMap::from([
                                        ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
                                        (
                                            "ROOT_PART_SIZE_AS_I64_MIB".to_string(),
                                            (partition_method_automatic_ratio_refcell.borrow().round() as i64 / 1048576).to_string().replace("'", r###"'"'"'"###).as_str(),
                                        )
                                    ]),
                                )
                                .unwrap());
                            }
                        }
                        "none" => {
                            if is_encrypted {
                                final_script.push_str(&strfmt::strfmt(
                                    auto_xfs::AUTOMATIC_HOME_NONE_XFS_LOCKED_INSTALLATION_PROG,
                                    &std::collections::HashMap::from([
                                        ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
                                        (
                                            "AUTO_LUKS_PASSWORD".to_string(),
                                            partition_method_automatic_luks_refcell.borrow().replace("'", r###"'"'"'"###).as_str(),
                                        )
                                    ]),
                                )
                                .unwrap());
                            } else {
                                final_script.push_str(&strfmt::strfmt(
                                    auto_xfs::AUTOMATIC_HOME_NONE_XFS_OPEN_INSTALLATION_PROG,
                                    &std::collections::HashMap::from([
                                        ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
                                    ]),
                                )
                                .unwrap());
                            }
                        }
                        _ => panic!()
                    }
                }
                _ => panic!()
            }
            //
            if is_encrypted {
                final_script.push_str(auto_basic::AUTOMATIC_LOCKED_PART_PIKAINSTALL_PROG);
            } else {
                final_script.push_str(auto_basic::AUTOMATIC_OPEN_PART_PIKAINSTALL_PROG);
            }
        }
        "manual" => {
            let is_encrypted = *partition_method_manual_luks_enabled_refcell.borrow();
            
            if is_encrypted {
                final_script.push_str(
r###"
                    
mkdir -p /tmp/PIKA_CRYPT/
touch /tmp/PIKA_CRYPT/crypttab
                    
"###
                );
                        
                for crypt_entry in partition_method_manual_crypttab_entry_array_refcell.borrow().iter() {
                    match &crypt_entry.password {
                        Some(p) => {
                            final_script.push_str(&strfmt::strfmt(
                                manual_basic::MANUAL_CRYPT_ENTRY_WITH_KEYFILE,
                                &std::collections::HashMap::from([
                                    (
                                        "MAP".to_string(),
                                        crypt_entry.map.replace("'", r###"'"'"'"###).as_str()
                                    ),
                                    (
                                        "UUID".to_string(),
                                        crypt_entry.uuid.replace("'", r###"'"'"'"###).as_str()
                                    ),
                                    (
                                        "LUKS_PASSWD".to_string(),
                                        p.replace("'", r###"'"'"'"###).as_str()
                                    )
                                ]),
                            )
                            .unwrap());
                        }
                        None => {
                            final_script.push_str(&strfmt::strfmt(
                                manual_basic::MANUAL_CRYPT_ENTRY,
                                &std::collections::HashMap::from([
                                    (
                                        "MAP".to_string(),
                                        crypt_entry.map.replace("'", r###"'"'"'"###).as_str()
                                    ),
                                    (
                                        "UUID".to_string(),
                                        crypt_entry.uuid.replace("'", r###"'"'"'"###).as_str()
                                    )
                                ]),
                            )
                            .unwrap());
                        }
                    }
                };
            }

            let mut did_make_swap_list = false;

            for fstab_entry in partition_method_manual_fstab_entry_array_refcell.borrow().iter() {
                if fstab_entry.mountpoint == "[SWAP]" {
                    if !did_make_swap_list {
                        final_script.push_str(
r###"

touch /tmp/pika-installer-gtk4-swaplist

"###
                        );
                        did_make_swap_list = true
                    }
                    final_script.push_str(&strfmt::strfmt(
                        manual_basic::MANUAL_SWAP_MOUNT_PROG,
                        &std::collections::HashMap::from([
                            (
                                "PART".to_string(),
                                fstab_entry.partition.part_name.replace("'", r###"'"'"'"###).as_str()
                            ),
                        ]),
                    )
                    .unwrap());
                } else if !fstab_entry.mountopts.is_empty() {
                    final_script.push_str(&strfmt::strfmt(
                        manual_basic::MANUAL_PARTITION_MOUNT_WITH_OPTS_PROG,
                        &std::collections::HashMap::from([
                            (
                                "PART".to_string(),
                                fstab_entry.partition.part_name.replace("'", r###"'"'"'"###).as_str()
                            ),
                            (
                                "MOUNTPOINT".to_string(),
                                fstab_entry.mountpoint.replace("'", r###"'"'"'"###).as_str()
                            ),
                            (
                                "OPTS".to_string(),
                                fstab_entry.mountopts.replace("'", r###"'"'"'"###).as_str()
                            ),
                        ]),
                    )
                    .unwrap());
                } else {
                    final_script.push_str(&strfmt::strfmt(
                        manual_basic::MANUAL_PARTITION_MOUNT_PROG,
                        &std::collections::HashMap::from([
                            (
                                "PART".to_string(),
                                fstab_entry.partition.part_name.replace("'", r###"'"'"'"###).as_str()
                            ),
                            (
                                "MOUNTPOINT".to_string(),
                                fstab_entry.mountpoint.replace("'", r###"'"'"'"###).as_str()
                            ),
                        ]),
                    )
                    .unwrap());
                }
            }

            if is_encrypted {
                final_script.push_str(manual_basic::MANUAL_LOCKED_PART_PIKAINSTALL_PROG);
            } else {
                final_script.push_str(manual_basic::MANUAL_OPEN_PART_PIKAINSTALL_PROG);
            }
        }
        _ => panic!()
    }

    final_script
}
