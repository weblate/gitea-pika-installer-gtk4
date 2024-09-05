pub const MANUAL_PARTITION_MOUNT_PROG: &str = r###"

mkdir -p "$PIKA_INSTALL_CHROOT_PATH"'{MOUNTPOINT}'
mount /dev/{PART} "$PIKA_INSTALL_CHROOT_PATH"'{MOUNTPOINT}'
"###;

pub const MANUAL_PARTITION_MOUNT_WITH_OPTS_PROG: &str = r###"

mkdir -p "$PIKA_INSTALL_CHROOT_PATH"'{MOUNTPOINT}'
mount -o '{OPTS}' /dev/'{PART}' "$PIKA_INSTALL_CHROOT_PATH"'{MOUNTPOINT}'
"###;

pub const MANUAL_SWAP_MOUNT_PROG: &str = r###"

echo /dev/'{PART}' >> /tmp/pika-installer-gtk4-swaplist
"###;

pub const MANUAL_CRYPT_ENTRY: &str = r###"

echo '{MAP} UUID={UUID} none luks,discard' >> /tmp/PIKA_CRYPT/crypttab
"###;

pub const MANUAL_CRYPT_ENTRY_WITH_KEYFILE: &str = r###"

echo '{MAP} UUID={UUID} /key-{MAP}.txt luks' >> /tmp/PIKA_CRYPT/crypttab
touch /tmp/PIKA_CRYPT/key-{MAP}.txt
openssl genrsa > /tmp/PIKA_CRYPT/key-{MAP}.txt
echo '{LUKS_PASSWD}' | cryptsetup luksAddKey UUID='{UUID}' /tmp/PIKA_CRYPT/key-{MAP}.txt -
"###;

pub const MANUAL_OPEN_PART_PIKAINSTALL_PROG: &str = r###"

if [ -z "$PIKA_INSTALL_XKB_VARIANT" ]
then
pikainstall --manual 1 -r "$PIKA_INSTALL_CHROOT_PATH" -l "$PIKA_INSTALL_LOCALE" -k "$PIKA_INSTALL_KEYMAP" -kl "$PIKA_INSTALL_XKB_LAYOUT" -t "$PIKA_INSTALL_TIMEZONE"
else
pikainstall --manual 1 -r "$PIKA_INSTALL_CHROOT_PATH" -l "$PIKA_INSTALL_LOCALE" -k "$PIKA_INSTALL_KEYMAP" -kl "$PIKA_INSTALL_XKB_LAYOUT" -t "$PIKA_INSTALL_TIMEZONE" -kv "$PIKA_INSTALL_XKB_VARIANT"
fi
"###;

pub const MANUAL_LOCKED_PART_PIKAINSTALL_PROG: &str = r###"

if [ -z "$PIKA_INSTALL_XKB_VARIANT" ]
then
pikainstall --manual 2 -r "$PIKA_INSTALL_CHROOT_PATH" -l "$PIKA_INSTALL_LOCALE" -k "$PIKA_INSTALL_KEYMAP" -kl "$PIKA_INSTALL_XKB_LAYOUT" -t "$PIKA_INSTALL_TIMEZONE"
else
pikainstall --manual 2 -r "$PIKA_INSTALL_CHROOT_PATH" -l "$PIKA_INSTALL_LOCALE" -k "$PIKA_INSTALL_KEYMAP" -kl "$PIKA_INSTALL_XKB_LAYOUT" -t "$PIKA_INSTALL_TIMEZONE" -kv "$PIKA_INSTALL_XKB_VARIANT"
fi
"###;