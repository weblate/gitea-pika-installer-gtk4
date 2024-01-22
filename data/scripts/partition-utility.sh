#! /bin/bash

if [[ "$1" = "get_block_devices" ]]
then
	lsblk -dn -o NAME
fi

if [[ "$1" = "get_block_size" ]]
then
	#disk_sector_num=$(cat /sys/block/"$2"/size)
	#echo ""$(expr $disk_sector_num / 2097152)" GB"
	lsblk -b --output SIZE -n -d /dev/"$2"
fi

if [[ "$1" = "check_home_encryption" ]]
then
	if blkid -o value -s TYPE $(lsblk -sJp | jq -r --arg dsk "$(df -P -h -T "$2/home" | awk 'END{print $1}')" '.blockdevices | .[] | select(.name == $dsk) | .children | .[0] | .name') | grep -i luks > /dev/null 2>&1
	then
		exit 0
	else
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

if [[ "$1" = "check_home_luks_passwd" ]]
then
	if printf "$3" | cryptsetup luksOpen --test-passphrase  UUID="$(blkid "$(lsblk -sJp | jq -r --arg dsk "$(df -P -h -T "$2"/home | awk 'END{print $1}')" '.blockdevices | .[] | select(.name == $dsk) | .children | .[0] | .name')" -s UUID -o value)" 
	then
		exit 0
	else
		exit 1
	fi
fi
