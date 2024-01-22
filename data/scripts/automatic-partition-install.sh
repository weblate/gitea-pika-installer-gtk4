#! /bin/bash

DISK="$(cat "/tmp/pika-installer-gtk4-target-auto.txt")"
LOCALE="$(cat "/tmp/pika-installer-gtk4-lang.txt")"
KEYBOARD="$(cat "/tmp/pika-installer-gtk4-target-auto.txt")"
TIMEZONE="$(cat "/tmp/pika-installer-gtk4-timezone.txt")"


if [[ ! -f "/tmp/pika-installer-gtk4-target-automatic-luks.txt" ]]
then
    sfdisk --delete /dev/${DISK}
    # Partition the drives
    parted -s -a optimal /dev/${DISK} mklabel gpt \
        mkpart "linux-efi"  1MiB 513Mib \
        mkpart "linux-boot" 513Mib 1537Mib \
        mkpart "linux-root" 1537Mib  42497Mib \
        mkpart "linux-home" 42497Mib  100% \
        print
    # add p to partition if it's nvme
    if echo ${DISK} | grep -i "nvme"
    then
        # Add filesystems
        mkfs.fat -F 32 /dev/${DISK}p1
        mkfs -t ext4 /dev/${DISK}p2
        mkfs -t btrfs /dev/${DISK}p3
        mkfs -t btrfs /dev/${DISK}p4
        # Begin Mounting
        mkdir -p /media/pika-install-mount
        mount /dev/${DISK}p3 /media/pika-install-mount/
        mkdir -p /media/pika-install-mount/home
        mount /dev/${DISK}p4 /media/pika-install-mount/home
        mkdir -p /media/pika-install-mount/boot
        mount /dev/${DISK}p2 /media/pika-install-mount/boot
        mkdir -p /media/pika-install-mount/boot/efi
        mount /dev/${DISK}p1 /media/pika-install-mount/boot/efi
        pikainstall -r /media/pika-install-mount/ -b /media/pika-install-mount/boot -e /media/pika-install-mount/boot/efi -l ${LOCALE} -k ${KEYBOARD} -t ${TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt
    else
        # Add filesystems
        mkfs.fat -F 32 /dev/${DISK}1
        mkfs -t ext4 /dev/${DISK}2
        mkfs -t btrfs /dev/${DISK}3
        mkfs -t btrfs /dev/${DISK}4
        # Begin Mounting
        mkdir -p /media/pika-install-mount
        mount /dev/${DISK}3 /media/pika-install-mount/
        mkdir -p /media/pika-install-mount/home
        mount /dev/${DISK}4 /media/pika-install-mount/home
        mkdir -p /media/pika-install-mount/boot
        mount /dev/${DISK}2 /media/pika-install-mount/boot
        mkdir -p /media/pika-install-mount/boot/efi
        mount /dev/${DISK}1 /media/pika-install-mount/boot/efi
        pikainstall -r /media/pika-install-mount/ -b /media/pika-install-mount/boot -e /media/pika-install-mount/boot/efi -l ${LOCALE} -k ${KEYBOARD} -t ${TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt
    fi
else
    LUKS_KEY="$(cat "/tmp/pika-installer-gtk4-target-automatic-luks.txt")"
fi