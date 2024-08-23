pub const automatic_standard_installation_prog: &str = r###"

echo 'PARTING' | nc -U $SOCKET_PATH || true

PIKA_INSTALL_AUTO_TARGET_DISK={AUTO_INSTALL_TARGET_DISK}

for part in $(/usr/lib/pika/pika-installer-gtk4/scripts/partition-utility.sh get_partitions | grep "$PIKA_INSTALL_AUTO_TARGET_DISK"); do
	PARTITION="/dev/$part"
	swapoff $PARTITION || true
done

wipefs -af /dev/"$AUTO_INSTALL_TARGET_DISK"
blockdev --rereadpt "$AUTO_INSTALL_TARGET_DISK"

"###;

pub const automatic_open_part_pikainstall_prog: &str = r###"

if [ -z $PIKA_INSTALL_KEYMAP_VARIANT ]
then
pikainstall -r $PIKA_INSTALL_CHROOT_PATH -l $PIKA_INSTALL_LOCALE -k $PIKA_INSTALL_KEYMAP_BASE -t $PIKA_INSTALL_TIMEZONE
else
pikainstall -r $PIKA_INSTALL_CHROOT_PATH -l $PIKA_INSTALL_LOCALE -k $PIKA_INSTALL_KEYMAP_BASE -t $PIKA_INSTALL_TIMEZONE -kv $PIKA_INSTALL_KEYMAP_VARIANT
fi
"###;

pub const automatic_locked_part_pikainstall_prog: &str = r###"

if [ -z $PIKA_INSTALL_KEYMAP_VARIANT ]
then
pikainstall -c $PIKA_INSTALL_AUTO_LUKS_PASSWORD -r $PIKA_INSTALL_CHROOT_PATH -l $PIKA_INSTALL_LOCALE -k $PIKA_INSTALL_KEYMAP_BASE -t $PIKA_INSTALL_TIMEZONE
else
pikainstall -c $PIKA_INSTALL_AUTO_LUKS_PASSWORD -r $PIKA_INSTALL_CHROOT_PATH -l $PIKA_INSTALL_LOCALE -k $PIKA_INSTALL_KEYMAP_BASE -t $PIKA_INSTALL_TIMEZONE -kv $PIKA_INSTALL_KEYMAP_VARIANT
fi
"###;