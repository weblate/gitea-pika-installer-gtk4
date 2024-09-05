pub const AUTOMATIC_STANDARD_INSTALLATION_PROG: &str = r###"

echo 'PARTING' | nc -U $SOCKET_PATH || true

PIKA_INSTALL_AUTO_TARGET_DISK={AUTO_INSTALL_TARGET_DISK}

# Unmount host partitions from chroot
umount "$PIKA_INSTALL_CHROOT_PATH/media/cdrom" || umount -lf "$PIKA_INSTALL_CHROOT_PATH/media/cdrom" || true
umount "$PIKA_INSTALL_CHROOT_PATH/dev" || umount -lf "$PIKA_INSTALL_CHROOT_PATH/dev" || true
umount "$PIKA_INSTALL_CHROOT_PATH/run" || umount -lf "$PIKA_INSTALL_CHROOT_PATH/run" || true
umount "$PIKA_INSTALL_CHROOT_PATH/proc" || umount -lf "$PIKA_INSTALL_CHROOT_PATH/proc" || true
umount "$PIKA_INSTALL_CHROOT_PATH/sys" || umount -lf "$PIKA_INSTALL_CHROOT_PATH/sys" || true

if echo "$PIKA_INSTALL_AUTO_TARGET_DISK" | grep -i "nvme"
then
	for part in $(/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh get_partitions | grep "$PIKA_INSTALL_AUTO_TARGET_DISK")p; do
		PARTITION="/dev/$part"
		swapoff $PARTITION || true
		umount -l $PARTITION || true
	done
else
	for part in $(/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh get_partitions | grep "$PIKA_INSTALL_AUTO_TARGET_DISK"); do
		PARTITION="/dev/$part"
		swapoff $PARTITION || true
		umount -l $PARTITION || true
	done
fi

wipefs -af /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK"
blockdev --rereadpt /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" || systemctl restart systemd-udevd.service

"###;

pub const AUTOMATIC_OPEN_PART_PIKAINSTALL_PROG: &str = r###"

if [ -z "$PIKA_INSTALL_XKB_VARIANT" ]
then
pikainstall -r $PIKA_INSTALL_CHROOT_PATH -l $PIKA_INSTALL_LOCALE -k $PIKA_INSTALL_KEYMAP -kl $PIKA_INSTALL_XKB_LAYOUT -t $PIKA_INSTALL_TIMEZONE
else
pikainstall -r $PIKA_INSTALL_CHROOT_PATH -l $PIKA_INSTALL_LOCALE -k $PIKA_INSTALL_KEYMAP -kl $PIKA_INSTALL_XKB_LAYOUT -t $PIKA_INSTALL_TIMEZONE -kv $PIKA_INSTALL_XKB_VARIANT
fi
"###;

pub const AUTOMATIC_LOCKED_PART_PIKAINSTALL_PROG: &str = r###"

if [ -z "$PIKA_INSTALL_XKB_VARIANT" ]
then
pikainstall -c "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" -r "$PIKA_INSTALL_CHROOT_PATH" -l "$PIKA_INSTALL_LOCALE" -k "$PIKA_INSTALL_KEYMAP" -kl "$PIKA_INSTALL_XKB_LAYOUT" -t "$PIKA_INSTALL_TIMEZONE"
else
pikainstall -c "$PIKA_INSTALL_AUTO_LUKS_PASSWORD" -r "$PIKA_INSTALL_CHROOT_PATH" -l "$PIKA_INSTALL_LOCALE" -k "$PIKA_INSTALL_KEYMAP" -kl "$PIKA_INSTALL_XKB_LAYOUT" -t "$PIKA_INSTALL_TIMEZONE" -kv "$PIKA_INSTALL_XKB_VARIANT"
fi
"###;