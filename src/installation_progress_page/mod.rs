use crate::{
    build_ui::{BlockDevice, CrypttabEntry, FstabEntry, PikaKeymap, PikaLocale},
    config::{MINIMUM_BOOT_BYTE_SIZE, MINIMUM_EFI_BYTE_SIZE},
    installer_stack_page,
};
use adw::prelude::*;
use glib::{clone, closure_local};
use gtk::{gio, glib};
use std::{cell::RefCell, fs, ops::Deref, path::Path, process::Command, rc::Rc};

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
) {
    let standard_installation_prog = r###"#! /bin/bash
set -e

PIKA_INSTALL_CHROOT_PATH={CHROOT_PATH}
PIKA_INSTALL_LOCALE={LOCALE}
PIKA_INSTALL_KEYMAP_BASE={KEYMAP_BASE}
PIKA_INSTALL_KEYMAP_VARIANT={KEYMAP_VARIANT}

touch "/tmp/pika-installer-gtk4-status.txt"
echo "PARTING" > "/tmp/pika-installer-gtk4-status.txt"

"###;

    let automatic_standard_installation_prog = r###"

PIKA_INSTALL_AUTO_TARGET_DISK={AUTO_INSTALL_TARGET_DISK}

for part in $(/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh get_partitions | grep "$PIKA_INSTALL_AUTO_TARGET_DISK"); do
	PARTITION="/dev/$part"
	swapoff $PARTITION || true
done

wipefs -af /dev/"$AUTO_INSTALL_TARGET_DISK"
blockdev --rereadpt "$AUTO_INSTALL_TARGET_DISK"

"###;

    //

    let automatic_home_subvol_btrfs_open_installation_prog = r###"

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  100%Mib \
print

if echo "$AUTO_INSTALL_TARGET_DISK" | grep -i "nvme"
then
#
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"p1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"p2
yes | mkfs.btrfs -f /dev/"$AUTO_INSTALL_TARGET_DISK"p3
sleep 2
# Begin Mounting
mkdir -p /var/cache/root-mnt
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p3 /var/cache/root-mnt
btrfs subvolume create /var/cache/root-mnt/@
btrfs subvolume create /var/cache/root-mnt/@home
#
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p3 $PIKA_INSTALL_CHROOT_PATH/ -o subvol=@
mkdir -p $PIKA_INSTALL_CHROOT_PATH/home
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p3 $PIKA_INSTALL_CHROOT_PATH/home -o subvol=@home
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
else
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"2
yes | mkfs.btrfs -f /dev/"$AUTO_INSTALL_TARGET_DISK"3
sleep 2
# Begin Mounting
mkdir -p /var/cache/root-mnt
mount /dev/"$AUTO_INSTALL_TARGET_DISK"3 /var/cache/root-mnt
btrfs subvolume create /var/cache/root-mnt/@
btrfs subvolume create /var/cache/root-mnt/@home
#
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/"$AUTO_INSTALL_TARGET_DISK"3 $PIKA_INSTALL_CHROOT_PATH/ -o subvol=@
mkdir -p $PIKA_INSTALL_CHROOT_PATH/home
mount /dev/"$AUTO_INSTALL_TARGET_DISK"3 $PIKA_INSTALL_CHROOT_PATH/home -o subvol=@home
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
fi

"###;

    let automatic_home_subvol_btrfs_locked_installation_prog = r###"

PIKA_INSTALL_AUTO_LUKS_PASSWORD={AUTO_LUKS_PASSWORD}

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  100%Mib \
print

# add p to partition if it's nvme
if echo "$AUTO_INSTALL_TARGET_DISK" | grep -i "nvme"
then
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"p1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"p2
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"p3
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"p3 crypt_root
yes | mkfs.btrfs -f /dev/mapper/crypt_root
sleep 2
# Begin Mounting
mkdir -p /var/cache/root-mnt
mount /dev/mapper/crypt_root /var/cache/root-mnt
btrfs subvolume create /var/cache/root-mnt/@
btrfs subvolume create /var/cache/root-mnt/@home
#
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/mapper/crypt_root $PIKA_INSTALL_CHROOT_PATH/ -o subvol=@
mkdir -p $PIKA_INSTALL_CHROOT_PATH/home
mount /dev/mapper/crypt_root $PIKA_INSTALL_CHROOT_PATH/home -o subvol=@home
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
else
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"2
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"3
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"3 crypt_root
yes | mkfs.btrfs -f /dev/mapper/crypt_root
sleep 2
# Begin Mounting
mkdir -p /var/cache/root-mnt
mount /dev/mapper/crypt_root /var/cache/root-mnt
btrfs subvolume create /var/cache/root-mnt/@
btrfs subvolume create /var/cache/root-mnt/@home
#
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/mapper/crypt_root $PIKA_INSTALL_CHROOT_PATH/ -o subvol=@
mkdir -p $PIKA_INSTALL_CHROOT_PATH/home
mount /dev/mapper/crypt_root $PIKA_INSTALL_CHROOT_PATH/home -o subvol=@home
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
fi

"###;

    //

    let automatic_home_part_btrfs_open_installation_prog = r###"

PIKA_INSTALL_AUTO_ROOT_SIZE={ROOT_PART_SIZE_AS_I64_MIB}

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  "$PIKA_INSTALL_AUTO_ROOT_SIZE"Mib \
mkpart "linux-home" "$PIKA_INSTALL_AUTO_ROOT_SIZE"Mib  100% \
print

if echo "$AUTO_INSTALL_TARGET_DISK" | grep -i "nvme"
then
#
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"p1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"p2
yes | mkfs.btrfs -f /dev/"$AUTO_INSTALL_TARGET_DISK"p3
yes | mkfs.btrfs -f /dev/"$AUTO_INSTALL_TARGET_DISK"p4
sleep 2
# Begin Mounting
mkdir -p /var/cache/root-mnt
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p3 /var/cache/root-mnt
btrfs subvolume create /var/cache/root-mnt/@
#
mkdir -p /var/cache/home-mnt
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p4 /var/cache/home-mnt
btrfs subvolume create /var/cache/home-mnt/@
#
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p3 $PIKA_INSTALL_CHROOT_PATH/ -o subvol=@
mkdir -p $PIKA_INSTALL_CHROOT_PATH/home
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p4 $PIKA_INSTALL_CHROOT_PATH/home -o subvol=@
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
else
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"2
yes | mkfs.btrfs -f /dev/"$AUTO_INSTALL_TARGET_DISK"3
yes | mkfs.btrfs -f /dev/"$AUTO_INSTALL_TARGET_DISK"4
sleep 2
# Begin Mounting
mkdir -p /var/cache/root-mnt
mount /dev/"$AUTO_INSTALL_TARGET_DISK"3 /var/cache/root-mnt
btrfs subvolume create /var/cache/root-mnt/@
#
mkdir -p /var/cache/home-mnt
mount /dev/"$AUTO_INSTALL_TARGET_DISK"4 /var/cache/home-mnt
btrfs subvolume create /var/cache/home-mnt/@
#
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/"$AUTO_INSTALL_TARGET_DISK"3 $PIKA_INSTALL_CHROOT_PATH/ -o subvol=@
mkdir -p $PIKA_INSTALL_CHROOT_PATH/home
mount /dev/"$AUTO_INSTALL_TARGET_DISK"4 $PIKA_INSTALL_CHROOT_PATH/home -o subvol=@
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
fi

"###;

    let automatic_home_part_btrfs_locked_installation_prog = r###"

PIKA_INSTALL_AUTO_LUKS_PASSWORD={AUTO_LUKS_PASSWORD}
PIKA_INSTALL_AUTO_ROOT_SIZE={ROOT_PART_SIZE_AS_I64_MIB}

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  "$PIKA_INSTALL_AUTO_ROOT_SIZE"Mib \
mkpart "linux-home" "$PIKA_INSTALL_AUTO_ROOT_SIZE"Mib  100% \
print

# add p to partition if it's nvme
if echo "$AUTO_INSTALL_TARGET_DISK" | grep -i "nvme"
then
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"p1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"p2
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"p3
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"p4
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"p3 crypt_root
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"p4 crypt_home
yes | mkfs.btrfs -f /dev/mapper/crypt_root
yes | mkfs.btrfs -f /dev/mapper/crypt_home
sleep 2
# Begin Mounting
mkdir -p /var/cache/root-mnt
mount /dev/mapper/crypt_root /var/cache/root-mnt
btrfs subvolume create /var/cache/root-mnt/@
#
mkdir -p /var/cache/home-mnt
mount /dev/mapper/crypt_home /var/cache/home-mnt
btrfs subvolume create /var/cache/home-mnt/@
#
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/mapper/crypt_root $PIKA_INSTALL_CHROOT_PATH/ -o subvol=@
mkdir -p $PIKA_INSTALL_CHROOT_PATH/home
mount /dev/mapper/crypt_home $PIKA_INSTALL_CHROOT_PATH/home -o subvol=@
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
else
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"2
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"3
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"4
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"3 crypt_root
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"4 crypt_home
yes | mkfs.btrfs -f /dev/mapper/crypt_root
yes | mkfs.btrfs -f /dev/mapper/crypt_home
sleep 2
# Begin Mounting
mkdir -p /var/cache/root-mnt
mount /dev/mapper/crypt_root /var/cache/root-mnt
btrfs subvolume create /var/cache/root-mnt/@
#
mkdir -p /var/cache/home-mnt
mount /dev/mapper/crypt_home /var/cache/home-mnt
btrfs subvolume create /var/cache/home-mnt/@
#
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/mapper/crypt_root $PIKA_INSTALL_CHROOT_PATH/ -o subvol=@
mkdir -p $PIKA_INSTALL_CHROOT_PATH/home
mount /dev/mapper/crypt_home $PIKA_INSTALL_CHROOT_PATH/home -o subvol=@
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
fi

"###;

    //

    let automatic_home_none_btrfs_open_installation_prog = r###"

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  100%Mib \
print

if echo "$AUTO_INSTALL_TARGET_DISK" | grep -i "nvme"
then
#
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"p1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"p2
yes | mkfs.btrfs -f /dev/"$AUTO_INSTALL_TARGET_DISK"p3
sleep 2
# Begin Mounting
mkdir -p /var/cache/root-mnt
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p3 /var/cache/root-mnt
btrfs subvolume create /var/cache/root-mnt/@
#
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p3 $PIKA_INSTALL_CHROOT_PATH/ -o subvol=@
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
else
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"2
yes | mkfs.btrfs -f /dev/"$AUTO_INSTALL_TARGET_DISK"3
sleep 2
# Begin Mounting
mkdir -p /var/cache/root-mnt
mount /dev/"$AUTO_INSTALL_TARGET_DISK"3 /var/cache/root-mnt
btrfs subvolume create /var/cache/root-mnt/@
#
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/"$AUTO_INSTALL_TARGET_DISK"3 $PIKA_INSTALL_CHROOT_PATH/ -o subvol=@
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
fi

"###;

    let automatic_home_none_btrfs_locked_installation_prog = r###"

PIKA_INSTALL_AUTO_LUKS_PASSWORD={AUTO_LUKS_PASSWORD}

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  100%Mib \
print

# add p to partition if it's nvme
if echo "$AUTO_INSTALL_TARGET_DISK" | grep -i "nvme"
then
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"p1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"p2
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"p3
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"p3 crypt_root
yes | mkfs.btrfs -f /dev/mapper/crypt_root
sleep 2
# Begin Mounting
mkdir -p /var/cache/root-mnt
mount /dev/mapper/crypt_root /var/cache/root-mnt
btrfs subvolume create /var/cache/root-mnt/@
#
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/mapper/crypt_root $PIKA_INSTALL_CHROOT_PATH/ -o subvol=@
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
else
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"2
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"3
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"3 crypt_root
yes | mkfs.btrfs -f /dev/mapper/crypt_root
sleep 2
# Begin Mounting
mkdir -p /var/cache/root-mnt
mount /dev/mapper/crypt_root /var/cache/root-mnt
btrfs subvolume create /var/cache/root-mnt/@
#
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/mapper/crypt_root $PIKA_INSTALL_CHROOT_PATH/ -o subvol=@
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
fi

"###;

    //

    let automatic_home_part_ext4_open_installation_prog = r###"

PIKA_INSTALL_AUTO_ROOT_SIZE={ROOT_PART_SIZE_AS_I64_MIB}

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  "$PIKA_INSTALL_AUTO_ROOT_SIZE"Mib \
mkpart "linux-home" "$PIKA_INSTALL_AUTO_ROOT_SIZE"Mib  100% \
print

if echo "$AUTO_INSTALL_TARGET_DISK" | grep -i "nvme"
then
#
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"p1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"p2
yes | mkfs.ext4 -F /dev/"$AUTO_INSTALL_TARGET_DISK"p3
yes | mkfs.ext4 -F /dev/"$AUTO_INSTALL_TARGET_DISK"p4
sleep 2
# Begin Mounting
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p3 $PIKA_INSTALL_CHROOT_PATH/
mkdir -p $PIKA_INSTALL_CHROOT_PATH/home
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p4 $PIKA_INSTALL_CHROOT_PATH/home
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
else
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"2
yes | mkfs.ext4 -F /dev/"$AUTO_INSTALL_TARGET_DISK"3
yes | mkfs.ext4 -F /dev/"$AUTO_INSTALL_TARGET_DISK"4
sleep 2
# Begin Mounting
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/"$AUTO_INSTALL_TARGET_DISK"3 $PIKA_INSTALL_CHROOT_PATH/
mkdir -p $PIKA_INSTALL_CHROOT_PATH/home
mount /dev/"$AUTO_INSTALL_TARGET_DISK"4 $PIKA_INSTALL_CHROOT_PATH/home
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
fi

"###;

    let automatic_home_part_ext4_locked_installation_prog = r###"

PIKA_INSTALL_AUTO_LUKS_PASSWORD={AUTO_LUKS_PASSWORD}
PIKA_INSTALL_AUTO_ROOT_SIZE={ROOT_PART_SIZE_AS_I64_MIB}

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  "$PIKA_INSTALL_AUTO_ROOT_SIZE"Mib \
mkpart "linux-home" "$PIKA_INSTALL_AUTO_ROOT_SIZE"Mib  100% \
print

# add p to partition if it's nvme
if echo "$AUTO_INSTALL_TARGET_DISK" | grep -i "nvme"
then
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"p1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"p2
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"p3
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"p4
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"p3 crypt_root
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"p4 crypt_home
yes | mkfs.ext4 -F /dev/mapper/crypt_root
yes | mkfs.ext4 -F /dev/mapper/crypt_home
sleep 2
# Begin Mounting
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/mapper/crypt_root $PIKA_INSTALL_CHROOT_PATH/
mkdir -p $PIKA_INSTALL_CHROOT_PATH/home
mount /dev/mapper/crypt_home $PIKA_INSTALL_CHROOT_PATH/home
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
else
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"2
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"3
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"4
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"3 crypt_root
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"4 crypt_home
yes | mkfs.ext4 -F /dev/mapper/crypt_root
yes | mkfs.ext4 -F /dev/mapper/crypt_home
sleep 2
# Begin Mounting
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/mapper/crypt_root $PIKA_INSTALL_CHROOT_PATH/
mkdir -p $PIKA_INSTALL_CHROOT_PATH/home
mount /dev/mapper/crypt_home $PIKA_INSTALL_CHROOT_PATH/home
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
fi

"###;

    //

    let automatic_home_none_ext4_open_installation_prog = r###"

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  100%Mib \
print

if echo "$AUTO_INSTALL_TARGET_DISK" | grep -i "nvme"
then
#
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"p1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"p2
yes | mkfs.ext4 -F /dev/"$AUTO_INSTALL_TARGET_DISK"p3
sleep 2
# Begin Mounting
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p3 $PIKA_INSTALL_CHROOT_PATH/
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
else
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"2
yes | mkfs.ext4 -F /dev/"$AUTO_INSTALL_TARGET_DISK"3
sleep 2
# Begin Mounting
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/"$AUTO_INSTALL_TARGET_DISK"3 $PIKA_INSTALL_CHROOT_PATH/
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
fi

"###;

    let automatic_home_none_ext4_locked_installation_prog = r###"

PIKA_INSTALL_AUTO_LUKS_PASSWORD={AUTO_LUKS_PASSWORD}

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  100%Mib \
print

# add p to partition if it's nvme
if echo "$AUTO_INSTALL_TARGET_DISK" | grep -i "nvme"
then
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"p1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"p2
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"p3
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"p3 crypt_root
yes | mkfs.ext4 -F /dev/mapper/crypt_root
sleep 2
# Begin Mounting
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/mapper/crypt_root $PIKA_INSTALL_CHROOT_PATH/
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
else
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"2
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"3
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"3 crypt_root
yes | mkfs.ext4 -F /dev/mapper/crypt_root
sleep 2
# Begin Mounting
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/mapper/crypt_root $PIKA_INSTALL_CHROOT_PATH/
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
fi

"###;

    //

    let automatic_home_part_xfs_open_installation_prog = r###"

PIKA_INSTALL_AUTO_ROOT_SIZE={ROOT_PART_SIZE_AS_I64_MIB}

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  "$PIKA_INSTALL_AUTO_ROOT_SIZE"Mib \
mkpart "linux-home" "$PIKA_INSTALL_AUTO_ROOT_SIZE"Mib  100% \
print

if echo "$AUTO_INSTALL_TARGET_DISK" | grep -i "nvme"
then
#
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"p1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"p2
yes | mkfs.xfs -f /dev/"$AUTO_INSTALL_TARGET_DISK"p3
yes | mkfs.xfs -f /dev/"$AUTO_INSTALL_TARGET_DISK"p4
sleep 2
# Begin Mounting
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p3 $PIKA_INSTALL_CHROOT_PATH/
mkdir -p $PIKA_INSTALL_CHROOT_PATH/home
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p4 $PIKA_INSTALL_CHROOT_PATH/home
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
else
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"2
yes | mkfs.xfs -f /dev/"$AUTO_INSTALL_TARGET_DISK"3
yes | mkfs.xfs -f /dev/"$AUTO_INSTALL_TARGET_DISK"4
sleep 2
# Begin Mounting
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/"$AUTO_INSTALL_TARGET_DISK"3 $PIKA_INSTALL_CHROOT_PATH/
mkdir -p $PIKA_INSTALL_CHROOT_PATH/home
mount /dev/"$AUTO_INSTALL_TARGET_DISK"4 $PIKA_INSTALL_CHROOT_PATH/home
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
fi

"###;

    let automatic_home_part_xfs_locked_installation_prog = r###"

PIKA_INSTALL_AUTO_LUKS_PASSWORD={AUTO_LUKS_PASSWORD}
PIKA_INSTALL_AUTO_ROOT_SIZE={ROOT_PART_SIZE_AS_I64_MIB}

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  "$PIKA_INSTALL_AUTO_ROOT_SIZE"Mib \
mkpart "linux-home" "$PIKA_INSTALL_AUTO_ROOT_SIZE"Mib  100% \
print

# add p to partition if it's nvme
if echo "$AUTO_INSTALL_TARGET_DISK" | grep -i "nvme"
then
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"p1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"p2
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"p3
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"p4
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"p3 crypt_root
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"p4 crypt_home
yes | mkfs.xfs -f /dev/mapper/crypt_root
yes | mkfs.xfs -f /dev/mapper/crypt_home
sleep 2
# Begin Mounting
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/mapper/crypt_root $PIKA_INSTALL_CHROOT_PATH/
mkdir -p $PIKA_INSTALL_CHROOT_PATH/home
mount /dev/mapper/crypt_home $PIKA_INSTALL_CHROOT_PATH/home
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
else
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"2
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"3
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"4
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"3 crypt_root
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"4 crypt_home
yes | mkfs.xfs -f /dev/mapper/crypt_root
yes | mkfs.xfs -f /dev/mapper/crypt_home
sleep 2
# Begin Mounting
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/mapper/crypt_root $PIKA_INSTALL_CHROOT_PATH/
mkdir -p $PIKA_INSTALL_CHROOT_PATH/home
mount /dev/mapper/crypt_home $PIKA_INSTALL_CHROOT_PATH/home
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
fi

"###;

    //

    let automatic_home_none_xfs_open_installation_prog = r###"

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  100%Mib \
print

if echo "$AUTO_INSTALL_TARGET_DISK" | grep -i "nvme"
then
#
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"p1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"p2
yes | mkfs.xfs -f /dev/"$AUTO_INSTALL_TARGET_DISK"p3
sleep 2
# Begin Mounting
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p3 $PIKA_INSTALL_CHROOT_PATH/
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
else
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"2
yes | mkfs.xfs -f /dev/"$AUTO_INSTALL_TARGET_DISK"3
sleep 2
# Begin Mounting
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/"$AUTO_INSTALL_TARGET_DISK"3 $PIKA_INSTALL_CHROOT_PATH/
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
fi

"###;

    let automatic_home_none_xfs_locked_installation_prog = r###"

PIKA_INSTALL_AUTO_LUKS_PASSWORD={AUTO_LUKS_PASSWORD}

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  100%Mib \
print

# add p to partition if it's nvme
if echo "$AUTO_INSTALL_TARGET_DISK" | grep -i "nvme"
then
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"p1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"p2
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"p3
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"p3 crypt_root
yes | mkfs.xfs -f /dev/mapper/crypt_root
sleep 2
# Begin Mounting
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/mapper/crypt_root $PIKA_INSTALL_CHROOT_PATH/
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"p1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
else
sleep 10
# Add filesystems
yes | mkfs -t vfat -F 32 /dev/"$AUTO_INSTALL_TARGET_DISK"1
yes | mkfs -t ext4 /dev/"$AUTO_INSTALL_TARGET_DISK"2
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v --type luks2 luksFormat /dev/"$AUTO_INSTALL_TARGET_DISK"3
printf "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" | cryptsetup -q -v luksOpen /dev/"$AUTO_INSTALL_TARGET_DISK"3 crypt_root
yes | mkfs.xfs -f /dev/mapper/crypt_root
sleep 2
# Begin Mounting
mkdir -p $PIKA_INSTALL_CHROOT_PATH
mount /dev/mapper/crypt_root $PIKA_INSTALL_CHROOT_PATH/
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot
mount /dev/"$AUTO_INSTALL_TARGET_DISK"2 $PIKA_INSTALL_CHROOT_PATH/boot
mkdir -p $PIKA_INSTALL_CHROOT_PATH/boot/efi
mount /dev/"$AUTO_INSTALL_TARGET_DISK"1 $PIKA_INSTALL_CHROOT_PATH/boot/efi
fi

"###;

    let script = strfmt::strfmt(
        standard_installation_prog,
        &std::collections::HashMap::from([
            ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
            (
                "LOCALE".to_string(),
                language_selection_text_refcell.borrow().name.as_str(),
            ),
            (
                "KEYMAP_BASE".to_string(),
                keymap_selection_text_refcell.borrow().name.as_str(),
            ),
            (
                "KEYMAP_VARIANT".to_string(),
                match &keymap_selection_text_refcell.borrow().variant {
                    Some(t) => t.as_str(),
                    None => "",
                },
            ),
        ]),
    )
    .unwrap();

    let script2 = strfmt::strfmt(
        automatic_home_part_ext4_locked_installation_prog,
        &std::collections::HashMap::from([
            ("CHROOT_PATH".to_string(), "/media/pikaos/installation"),
            (
                "AUTO_LUKS_PASSWORD".to_string(),
                partition_method_automatic_luks_refcell.borrow().as_str(),
            ),
            (
                "ROOT_PART_SIZE_AS_I64_MIB".to_string(),
                (partition_method_automatic_ratio_refcell.borrow().round() as i64 / 1048576).to_string().as_str(),
            ),
        ]),
    )
    .unwrap();

    println!("{}", script2)
}
