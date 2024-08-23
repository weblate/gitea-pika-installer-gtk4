pub const manual_open_part_pikainstall_prog: &str = r###"

if [ -z $PIKA_INSTALL_KEYMAP_VARIANT ]
then
pikainstall --manual 1 -r $PIKA_INSTALL_CHROOT_PATH -l $PIKA_INSTALL_LOCALE -k $PIKA_INSTALL_KEYMAP_BASE -t $PIKA_INSTALL_TIMEZONE
else
pikainstall --manual 1 -r $PIKA_INSTALL_CHROOT_PATH -l $PIKA_INSTALL_LOCALE -k $PIKA_INSTALL_KEYMAP_BASE -t $PIKA_INSTALL_TIMEZONE
fi -kv $PIKA_INSTALL_KEYMAP_VARIANT
"###;

pub const manual_locked_part_pikainstall_prog: &str = r###"

if [ -z $PIKA_INSTALL_KEYMAP_VARIANT ]
then
pikainstall --manual 2 -r $PIKA_INSTALL_CHROOT_PATH -l $PIKA_INSTALL_LOCALE -k $PIKA_INSTALL_KEYMAP_BASE -t $PIKA_INSTALL_TIMEZONE
else
pikainstall --manual 2-r $PIKA_INSTALL_CHROOT_PATH -l $PIKA_INSTALL_LOCALE -k $PIKA_INSTALL_KEYMAP_BASE -t $PIKA_INSTALL_TIMEZONE
fi -kv $PIKA_INSTALL_KEYMAP_VARIANT
"###;