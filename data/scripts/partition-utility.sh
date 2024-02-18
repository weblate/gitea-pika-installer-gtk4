#! /bin/bash

export LANG=en_US.UTF8

if [[ "$1" = "get_block_devices" ]]
then
	lsblk -dn -o NAME | grep -v -i -E 'loop|zram|sr|cdrom|portal'
fi

if [[ "$1" = "get_block_size" ]]
then
	#disk_sector_num=$(cat /sys/block/"$2"/size)
	#echo ""$(expr $disk_sector_num / 2097152)" GB"
	lsblk -b --output SIZE -n -d /dev/"$2"
fi

if [[ "$1" = "has_encryption" ]]
then
	if blkid -o value -s TYPE $(lsblk -sJp | jq -r --arg dsk /dev/"$2" '.blockdevices | .[] | select(.name == $dsk) | .children | .[0] | .name') | grep -i luks > /dev/null 2>&1
	then
		echo "$2 has encryption"
		exit 0
	else
	  echo "$2 is unencrypted"
		exit 1
	fi
fi

if [[ "$1" = "home_not_root" ]]
then
	if [[ $(blkid "$(df -P -h -T "$2" | awk 'END{print $1}')" -s UUID -o value) == $(blkid "$(df -P -h -T "$2/home" | awk 'END{print $1}')" -s UUID -o value) ]]
	then
		exit 1
	else
		exit 0
	fi
fi

if [[ "$1" = "get_partitions" ]]
then
  lsblk -ln -o NAME,TYPE | grep -E "part|crypt|lvm" |  awk '{print $1}' | while read i ; do
    if lsblk -ln -o NAME,TYPE | grep "$i" | grep "crypt" > /dev/null 2>&1
    then
        echo "mapper/$(lsblk -ln -o NAME,TYPE | grep "$i" | awk '{print $1}')"
    fi

    if lsblk -ln -o NAME,TYPE | grep "$i" | grep "lvm" > /dev/null 2>&1
    then
        echo "mapper/$(lsblk -ln -o NAME,TYPE | grep "$i" | awk '{print $1}')"
    fi

    if lsblk -ln -o NAME,TYPE | grep "$i" | grep "part" > /dev/null 2>&1
    then
        lsblk -ln -o NAME,TYPE | grep "$i" |  awk '{print $1}'
    fi
  done
fi

if [[ "$1" = "get_part_fs" ]]
then
  lsblk -ln -o NAME,FSTYPE | grep "$2" | awk '{print $2}'
fi

if [[ "$1" = "get_part_size" ]]
then
  lsblk -b --output SIZE -n -d /dev/"$2"
fi

if [[ "$1" = "home_not_boot" ]]
then
        if [[ $(blkid "$(df -P -h -T "$2/boot" | awk 'END{print $1}')" -s UUID -o value) == $(blkid "$(df -P -h -T "$2/home" | awk 'END{print $1}')" -s UUID -o value) ]]
        then
                exit 1
        else
                exit 0
        fi
fi

if [[ "$1" = "home_not_efi" ]]
then
        if [[ $(blkid "$(df -P -h -T "$2/boot/efi" | awk 'END{print $1}')" -s UUID -o value) == $(blkid "$(df -P -h -T "$2/home" | awk 'END{print $1}')" -s UUID -o value) ]]
        then
                exit 1
        else
                exit 0
        fi
fi

if [[ "$1" = "root_not_boot" ]]
then
        if [[ $(blkid "$(df -P -h -T "$2/boot" | awk 'END{print $1}')" -s UUID -o value) == $(blkid "$(df -P -h -T "$2" | awk 'END{print $1}')" -s UUID -o value) ]]
        then
                exit 1
        else
                exit 0
        fi
fi


if [[ "$1" = "root_not_efi" ]]
then
        if [[ $(blkid "$(df -P -h -T "$2/boot/efi" | awk 'END{print $1}')" -s UUID -o value) == $(blkid "$(df -P -h -T "$2" | awk 'END{print $1}')" -s UUID -o value) ]]
        then
                exit 1
        else
                exit 0
        fi
fi

if [[ "$1" = "boot_not_efi" ]]
then
        if [[ $(blkid "$(df -P -h -T "$2/boot" | awk 'END{print $1}')" -s UUID -o value) == $(blkid "$(df -P -h -T "$2/boot/efi" | awk 'END{print $1}')" -s UUID -o value) ]]
        then
                exit 1
        else
                exit 0
        fi
fi

if [[ "$1" = "test_luks_passwd" ]]
then
	if printf "$3" | cryptsetup luksOpen --test-passphrase  UUID="$(blkid "$(lsblk -sJp | jq -r --arg dsk /dev/"$2" '.blockdevices | .[] | select(.name == $dsk) | .children | .[0] | .name')" -s UUID -o value)"
	then
		exit 0
	else
		exit 1
	fi
fi
