// BTRFS WITH SUBVOL
pub const automatic_home_subvol_btrfs_open_installation_prog: &str = r###"
parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  100%Mib \
print

blockdev --rereadpt "$AUTO_INSTALL_TARGET_DISK"

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
echo 'MOUNTING' | nc -U $SOCKET_PATH || true
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
echo 'MOUNTING' | nc -U $SOCKET_PATH || true
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

pub const automatic_home_subvol_btrfs_locked_installation_prog: &str =  r###"

PIKA_INSTALL_AUTO_LUKS_PASSWORD={AUTO_LUKS_PASSWORD}

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  100%Mib \
print

blockdev --rereadpt "$AUTO_INSTALL_TARGET_DISK"

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
echo 'MOUNTING' | nc -U $SOCKET_PATH || true
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
echo 'MOUNTING' | nc -U $SOCKET_PATH || true
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

// BTRFS WITH HOME PART

pub const automatic_home_part_btrfs_open_installation_prog: &str =  r###"

PIKA_INSTALL_AUTO_ROOT_SIZE={ROOT_PART_SIZE_AS_I64_MIB}

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  "$PIKA_INSTALL_AUTO_ROOT_SIZE"Mib \
mkpart "linux-home" "$PIKA_INSTALL_AUTO_ROOT_SIZE"Mib  100% \
print

blockdev --rereadpt "$AUTO_INSTALL_TARGET_DISK"

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
echo 'MOUNTING' | nc -U $SOCKET_PATH || true
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
echo 'MOUNTING' | nc -U $SOCKET_PATH || true
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

pub const automatic_home_part_btrfs_locked_installation_prog: &str =  r###"

PIKA_INSTALL_AUTO_LUKS_PASSWORD={AUTO_LUKS_PASSWORD}
PIKA_INSTALL_AUTO_ROOT_SIZE={ROOT_PART_SIZE_AS_I64_MIB}

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  "$PIKA_INSTALL_AUTO_ROOT_SIZE"Mib \
mkpart "linux-home" "$PIKA_INSTALL_AUTO_ROOT_SIZE"Mib  100% \
print

blockdev --rereadpt "$AUTO_INSTALL_TARGET_DISK"

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
echo 'MOUNTING' | nc -U $SOCKET_PATH || true
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
echo 'MOUNTING' | nc -U $SOCKET_PATH || true
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

// BTRFS WITH NO HOME

pub const automatic_home_none_btrfs_open_installation_prog: &str =  r###"

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  100%Mib \
print

blockdev --rereadpt "$AUTO_INSTALL_TARGET_DISK"

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
echo 'MOUNTING' | nc -U $SOCKET_PATH || true
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
echo 'MOUNTING' | nc -U $SOCKET_PATH || true
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

pub const automatic_home_none_btrfs_locked_installation_prog: &str =  r###"

PIKA_INSTALL_AUTO_LUKS_PASSWORD={AUTO_LUKS_PASSWORD}

parted -s -a optimal /dev/"$PIKA_INSTALL_AUTO_TARGET_DISK" mklabel gpt \
mkpart "linux-efi"  1MiB 500Mib \
mkpart "linux-boot" 500Mib 1500Mib \
mkpart "linux-root" 1500Mib  100%Mib \
print

blockdev --rereadpt "$AUTO_INSTALL_TARGET_DISK"

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
echo 'MOUNTING' | nc -U $SOCKET_PATH || true
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
echo 'MOUNTING' | nc -U $SOCKET_PATH || true
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