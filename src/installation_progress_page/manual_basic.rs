pub const manual_partition_mount_prog: &str = r###"

mkdir -p "$PIKA_INSTALL_CHROOT_PATH"'{MOUNTPOINT}'
mount {PART} "$PIKA_INSTALL_CHROOT_PATH"'{MOUNTPOINT}'
"###;

pub const manual_partition_mount_with_opts_prog: &str = r###"

mkdir -p "$PIKA_INSTALL_CHROOT_PATH"'{MOUNTPOINT}'
mount -o '{OPTS}' '{PART}' "$PIKA_INSTALL_CHROOT_PATH"'{MOUNTPOINT}'
"###;

pub const manual_swap_mount_prog: &str = r###"

echo '{PART}' >> /tmp/pika-installer-gtk4-swaplist
"###;

pub const manual_crypt_entry: &str = r###"

echo '{MAP} UUID={UUID} none luks,discard' >> /tmp/PIKA_CRYPT/crypttab
"###;

pub const manual_crypt_entry_with_keyfile: &str = r###"

echo '{MAP} UUID={UUID} /key-{MAP}.txt luks' >> /tmp/PIKA_CRYPT/crypttab
touch /tmp/PIKA_CRYPT/key-{MAP}.txt
openssl genrsa > /tmp/PIKA_CRYPT/key-{MAP}.txt
echo '{LUKS_PASSWD}' | cryptsetup luksAddKey UUID='{UUID}' /tmp/PIKA_CRYPT/key-{MAP}.txt -
"###;

pub const manual_open_part_pikainstall_prog: &str = r###"

if [ -z $PIKA_INSTALL_KEYMAP_VARIANT ]
then
pikainstall --manual 1 -r $PIKA_INSTALL_CHROOT_PATH -l $PIKA_INSTALL_LOCALE -k $PIKA_INSTALL_KEYMAP_BASE -t $PIKA_INSTALL_TIMEZONE
else
pikainstall --manual 1 -r $PIKA_INSTALL_CHROOT_PATH -l $PIKA_INSTALL_LOCALE -k $PIKA_INSTALL_KEYMAP_BASE -t $PIKA_INSTALL_TIMEZONE -kv $PIKA_INSTALL_KEYMAP_VARIANT
fi
"###;

pub const manual_locked_part_pikainstall_prog: &str = r###"

if [ -z $PIKA_INSTALL_KEYMAP_VARIANT ]
then
pikainstall --manual 2 -r $PIKA_INSTALL_CHROOT_PATH -l $PIKA_INSTALL_LOCALE -k $PIKA_INSTALL_KEYMAP_BASE -t $PIKA_INSTALL_TIMEZONE
else
pikainstall --manual 2 -r $PIKA_INSTALL_CHROOT_PATH -l $PIKA_INSTALL_LOCALE -k $PIKA_INSTALL_KEYMAP_BASE -t $PIKA_INSTALL_TIMEZONE -kv $PIKA_INSTALL_KEYMAP_VARIANT
fi
"###;