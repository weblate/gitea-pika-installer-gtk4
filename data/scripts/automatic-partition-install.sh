#! /bin/bash

export LANG=en_US.UTF8

set -e

S_DISK="$(cat "/tmp/pika-installer-gtk4-target-auto.txt")"
S_LOCALE="$(cat "/tmp/pika-installer-gtk4-lang.txt")"
S_KEYBOARD="$(cat "/tmp/pika-installer-gtk4-keyboard.txt")"
S_TIMEZONE="$(cat "/tmp/pika-installer-gtk4-timezone.txt")"

p3_size=$(echo "scale=2 ; $(cat /tmp/pika-installer-p3-size.txt) / 1024 / 1024" | bc | cut -f1 -d".")

touch "/tmp/pika-installer-gtk4-status-parting.txt"

if [[ ! -f "/tmp/pika-installer-gtk4-target-automatic-luks.txt" ]]
then
    for part in $(sudo /usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh get_partitions | grep ${S_DISK}); do
    	PARTITION="/dev/$part"
    	sudo swapoff $PARTITION || true
    done
    wipefs -af /dev/${S_DISK}
    # Partition the drives
    parted -s -a optimal /dev/${S_DISK} mklabel gpt \
        mkpart "linux-efi"  1MiB 513Mib \
        mkpart "linux-boot" 513Mib 1537Mib \
        mkpart "linux-root" 1537Mib  "$p3_size"Mib \
        mkpart "linux-home" "$p3_size"Mib  100% \
        print
    # add p to partition if it's nvme
    if echo ${S_DISK} | grep -i "nvme"
    then
        #
        sleep 10
        # Add filesystems
        yes | mkfs -t vfat -F 32 /dev/${S_DISK}p1
        yes | mkfs -t ext4 /dev/${S_DISK}p2
        yes | mkfs.btrfs -f /dev/${S_DISK}p3
        yes | mkfs.btrfs -f /dev/${S_DISK}p4
        sleep 2
        # Begin Mounting
        mkdir -p /media/pika-install-mount
        mount /dev/${S_DISK}p3 /media/pika-install-mount/
        mkdir -p /media/pika-install-mount/home
        mount /dev/${S_DISK}p4 /media/pika-install-mount/home
        mkdir -p /media/pika-install-mount/boot
        mount /dev/${S_DISK}p2 /media/pika-install-mount/boot
        mkdir -p /media/pika-install-mount/boot/efi
        mount /dev/${S_DISK}p1 /media/pika-install-mount/boot/efi
        pikainstall -r /media/pika-install-mount/ -l ${S_LOCALE} -k ${S_KEYBOARD} -t ${S_TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt || touch /tmp/pika-installer-gtk4-fail.txt && exit 1
    else
        sleep 10
        # Add filesystems
        yes | mkfs -t vfat -F 32 /dev/${S_DISK}1
        yes | mkfs -t ext4 /dev/${S_DISK}2
        yes | mkfs.btrfs -f /dev/${S_DISK}3
        yes | mkfs.btrfs -f /dev/${S_DISK}4
        sleep 2
        # Begin Mounting
        mkdir -p /media/pika-install-mount
        mount /dev/${S_DISK}3 /media/pika-install-mount/
        mkdir -p /media/pika-install-mount/home
        mount /dev/${S_DISK}4 /media/pika-install-mount/home
        mkdir -p /media/pika-install-mount/boot
        mount /dev/${S_DISK}2 /media/pika-install-mount/boot
        mkdir -p /media/pika-install-mount/boot/efi
        mount /dev/${S_DISK}1 /media/pika-install-mount/boot/efi
        pikainstall -r /media/pika-install-mount/ -l ${S_LOCALE} -k ${S_KEYBOARD} -t ${S_TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt || touch /tmp/pika-installer-gtk4-fail.txt && exit 1
    fi
else
    S_LUKS_KEY="$(cat "/tmp/pika-installer-gtk4-target-automatic-luks.txt")"
    for part in $(sudo /usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh get_partitions | grep ${S_DISK}); do
    	PARTITION="/dev/$part"
    	sudo swapoff $PARTITION || true
    done
    wipefs -af /dev/${S_DISK}
    # Partition the drives
    parted -s -a optimal /dev/${S_DISK} mklabel gpt \
        mkpart "linux-efi"  1MiB 513Mib \
        mkpart "linux-boot" 513Mib 1537Mib \
        mkpart "linux-root" 1537Mib  "$p3_size"Mib \
        mkpart "linux-home" "$p3_size"Mib  100% \
        print
    # add p to partition if it's nvme
    if echo ${S_DISK} | grep -i "nvme"
    then
        sleep 10
        # Add filesystems
        yes | mkfs -t vfat -F 32 /dev/${S_DISK}p1
        yes | mkfs -t ext4 /dev/${S_DISK}p2
        printf ${S_LUKS_KEY} | cryptsetup -q -v --type luks2 luksFormat /dev/${S_DISK}p3
        printf ${S_LUKS_KEY} | cryptsetup -q -v --type luks2 luksFormat /dev/${S_DISK}p4
        printf ${S_LUKS_KEY} | cryptsetup -q -v luksOpen /dev/${S_DISK}p3 crypt_root
        printf ${S_LUKS_KEY} | cryptsetup -q -v luksOpen /dev/${S_DISK}p4 crypt_home
        yes | mkfs.btrfs -f /dev/mapper/crypt_root
        yes | mkfs.btrfs -f /dev/mapper/crypt_home
        sleep 2
        # Begin Mounting
        mkdir -p /media/pika-install-mount
        mount /dev/mapper/crypt_root /media/pika-install-mount/
        mkdir -p /media/pika-install-mount/home
        mount /dev/mapper/crypt_home /media/pika-install-mount/home
        mkdir -p /media/pika-install-mount/boot
        mount /dev/${S_DISK}p2 /media/pika-install-mount/boot
        mkdir -p /media/pika-install-mount/boot/efi
        mount /dev/${S_DISK}p1 /media/pika-install-mount/boot/efi
        pikainstall -r /media/pika-install-mount/ -c ${S_LUKS_KEY} -l ${S_LOCALE} -k ${S_KEYBOARD} -t ${S_TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt || touch /tmp/pika-installer-gtk4-fail.txt && exit 1
    else
        sleep 10
        # Add filesystems
        yes | mkfs -t vfat -F 32 /dev/${S_DISK}1
        yes | mkfs -t ext4 /dev/${S_DISK}2
        printf ${S_LUKS_KEY} | cryptsetup -q -v --type luks2 luksFormat /dev/${S_DISK}3
        printf ${S_LUKS_KEY} | cryptsetup -q -v --type luks2 luksFormat /dev/${S_DISK}4
        printf ${S_LUKS_KEY} | cryptsetup -q -v luksOpen /dev/${S_DISK}3 crypt_root
        printf ${S_LUKS_KEY} | cryptsetup -q -v luksOpen /dev/${S_DISK}4 crypt_home
        yes | mkfs.btrfs -f /dev/mapper/crypt_root
        yes | mkfs.btrfs -f /dev/mapper/crypt_home
        sleep 2
        # Begin Mounting
        mkdir -p /media/pika-install-mount
        mount /dev/mapper/crypt_root /media/pika-install-mount/
        mkdir -p /media/pika-install-mount/home
        mount /dev/mapper/crypt_home /media/pika-install-mount/home
        mkdir -p /media/pika-install-mount/boot
        mount /dev/${S_DISK}2 /media/pika-install-mount/boot
        mkdir -p /media/pika-install-mount/boot/efi
        mount /dev/${S_DISK}1 /media/pika-install-mount/boot/efi
        pikainstall -r /media/pika-install-mount/ -c ${S_LUKS_KEY} -l ${S_LOCALE} -k ${S_KEYBOARD} -t ${S_TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt || touch /tmp/pika-installer-gtk4-fail.txt && exit 1
    fi
fi