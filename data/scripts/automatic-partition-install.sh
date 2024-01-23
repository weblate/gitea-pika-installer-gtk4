#! /bin/bash

set -e

DISK="$(cat "/tmp/pika-installer-gtk4-target-auto.txt")"
LOCALE="$(cat "/tmp/pika-installer-gtk4-lang.txt")"
KEYBOARD="$(cat "/tmp/pika-installer-gtk4-target-auto.txt")"
TIMEZONE="$(cat "/tmp/pika-installer-gtk4-timezone.txt")"

touch "/tmp/pika-installer-gtk4-status-parting.txt"

if [[ ! -f "/tmp/pika-installer-gtk4-target-automatic-luks.txt" ]]
then
    wipefs -a /dev/${DISK}
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
        #
        sleep 10
        # Add filesystems
        yes | mkfs -t vfat -F 32 /dev/${DISK}p1
        yes | mkfs -t ext4 /dev/${DISK}p2
        yes | mkfs.btrfs -f /dev/${DISK}p3
        yes | mkfs.btrfs -f /dev/${DISK}p4
        sleep 2
        # Begin Mounting
        mkdir -p /media/pika-install-mount
        mount /dev/${DISK}p3 /media/pika-install-mount/
        mkdir -p /media/pika-install-mount/home
        mount /dev/${DISK}p4 /media/pika-install-mount/home
        mkdir -p /media/pika-install-mount/boot
        mount /dev/${DISK}p2 /media/pika-install-mount/boot
        mkdir -p /media/pika-install-mount/boot/efi
        mount /dev/${DISK}p1 /media/pika-install-mount/boot/efi
        pikainstall -r /media/pika-install-mount/ -l ${LOCALE} -k ${KEYBOARD} -t ${TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt
    else
        sleep 10
        # Add filesystems
        yes | mkfs -t vfat -F 32 /dev/${DISK}1
        yes | mkfs -t ext4 /dev/${DISK}2
        yes | mkfs.btrfs -f /dev/${DISK}3
        yes | mkfs.btrfs -f /dev/${DISK}4
        sleep 2
        # Begin Mounting
        mkdir -p /media/pika-install-mount
        mount /dev/${DISK}3 /media/pika-install-mount/
        mkdir -p /media/pika-install-mount/home
        mount /dev/${DISK}4 /media/pika-install-mount/home
        mkdir -p /media/pika-install-mount/boot
        mount /dev/${DISK}2 /media/pika-install-mount/boot
        mkdir -p /media/pika-install-mount/boot/efi
        mount /dev/${DISK}1 /media/pika-install-mount/boot/efi
        pikainstall -r /media/pika-install-mount/ -l ${LOCALE} -k ${KEYBOARD} -t ${TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt -l ${LOCALE} -k ${KEYBOARD} -t ${TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt
    fi
else
    LUKS_KEY="$(cat "/tmp/pika-installer-gtk4-target-automatic-luks.txt")"
    wipefs -a /dev/${DISK}
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
        sleep 10
        # Add filesystems
        yes | mkfs -t vfat -F 32 /dev/${DISK}p1
        yes | mkfs -t ext4 /dev/${DISK}p2
        printf ${LUKS_KEY} | cryptsetup -q -v --type luks2 luksFormat /dev/${DISK}p3
        printf ${LUKS_KEY} | cryptsetup -q -v --type luks2 luksFormat /dev/${DISK}p4
        printf ${LUKS_KEY} | cryptsetup -q -v luksOpen /dev/${DISK}p3 crypt_root
        printf ${LUKS_KEY} | cryptsetup -q -v luksOpen /dev/${DISK}p4 crypt_home
        yes | mkfs.btrfs -f /dev/mapper/crypt_root
        yes | mkfs.btrfs -f /dev/mapper/crypt_home
        sleep 2
        # Begin Mounting
        mkdir -p /media/pika-install-mount
        mount /dev/mapper/crypt_root /media/pika-install-mount/
        mkdir -p /media/pika-install-mount/home
        mount /dev/mapper/crypt_home /media/pika-install-mount/home
        mkdir -p /media/pika-install-mount/boot
        mount /dev/${DISK}p2 /media/pika-install-mount/boot
        mkdir -p /media/pika-install-mount/boot/efi
        mount /dev/${DISK}p1 /media/pika-install-mount/boot/efi
        pikainstall -r /media/pika-install-mount/ -c ${LUKS_KEY} -l ${LOCALE} -k ${KEYBOARD} -t ${TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt -l ${LOCALE} -k ${KEYBOARD} -t ${TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt
    else
        sleep 10
        # Add filesystems
        yes | mkfs -t vfat -F 32 /dev/${DISK}1
        yes | mkfs -t ext4 /dev/${DISK}2
        printf ${LUKS_KEY} | cryptsetup -q -v --type luks2 luksFormat /dev/${DISK}3
        printf ${LUKS_KEY} | cryptsetup -q -v --type luks2 luksFormat /dev/${DISK}4
        printf ${LUKS_KEY} | cryptsetup -q -v luksOpen /dev/${DISK}3 crypt_root
        printf ${LUKS_KEY} | cryptsetup -q -v luksOpen /dev/${DISK}4 crypt_home
        yes | mkfs.btrfs -f /dev/mapper/crypt_root
        yes | mkfs.btrfs -f /dev/mapper/crypt_home
        sleep 2
        # Begin Mounting
        mkdir -p /media/pika-install-mount
        mount /dev/mapper/crypt_root /media/pika-install-mount/
        mkdir -p /media/pika-install-mount/home
        mount /dev/mapper/crypt_home /media/pika-install-mount/home
        mkdir -p /media/pika-install-mount/boot
        mount /dev/${DISK}2 /media/pika-install-mount/boot
        mkdir -p /media/pika-install-mount/boot/efi
        mount /dev/${DISK}1 /media/pika-install-mount/boot/efi
        pikainstall -r /media/pika-install-mount/ -c ${LUKS_KEY} -l ${LOCALE} -k ${KEYBOARD} -t ${TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt -l ${LOCALE} -k ${KEYBOARD} -t ${TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt
    fi
fi