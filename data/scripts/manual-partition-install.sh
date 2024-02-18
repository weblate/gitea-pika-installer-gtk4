#! /bin/bash

set -e

LOCALE="$(cat "/tmp/pika-installer-gtk4-lang.txt")"
KEYBOARD="$(cat "/tmp/pika-installer-gtk4-keyboard.txt")"
TIMEZONE="$(cat "/tmp/pika-installer-gtk4-timezone.txt")"

touch "/tmp/pika-installer-gtk4-status-parting.txt"

rm -rf /tmp/pika-installer-gtk4-swaplist || true

if ls /tmp/pika-installer-gtk4-target-manual-luks-p*.json
then
  rm -rf /tmp/pika-installer-gtk4-crypttab || true
  touch /tmp/pika-installer-gtk4-crypttab

  for cryptentry in /tmp/pika-installer-gtk4-target-manual-luks-p*.json; do
    if [[ -z $(jq -r .password $cryptentry) ]]
    then
      LUKS=$(jq -r .partition $cryptentry)
      MAP=$(jq -r .partition $cryptentry | cut -d "/" -f2-)
      UUID="$(blkid "$(lsblk -sJp | jq -r --arg dsk /dev/"$LUKS" '.blockdevices | .[] | select(.name == $dsk) | .children | .[0] | .name')" -s UUID -o value)"
      echo "$MAP UUID="$UUID" none luks,discard" >> /tmp/pika-installer-gtk4-crypttab
    else
      LUKS=$(jq -r .partition $cryptentry)
      MAP=$(jq -r .partition $cryptentry | cut -d "/" -f2-)
      UUID="$(blkid "$(lsblk -sJp | jq -r --arg dsk /dev/"$LUKS" '.blockdevices | .[] | select(.name == $dsk) | .children | .[0] | .name')" -s UUID -o value)"
      LUKS_PASSWD=$(jq -r .password $cryptentry)
      echo "$MAP UUID="$UUID" /key-"$MAP".txt luks" >> /tmp/pika-installer-gtk4-crypttab
      touch /key-"$MAP".txt
      openssl genrsa > /key-"$MAP".txt
      echo $LUKS_PASSWD | cryptsetup luksAddKey UUID=$UUID	/key-"$MAP".txt -
    fi
  done
fi

for drivemount in /tmp/pika-installer-gtk4-target-manual-p*.json; do
	PARTITION="/dev/$(jq -r .partition $drivemount)"
	MOUNTPOINT="/media/pika-install-mount/$(jq -r .mountpoint $drivemount)"
	MOUNTOPT=$(jq -r .mountopt $drivemount)
	if [[ $MOUNTPOINT = '/media/pika-install-mount/[SWAP]' ]]
  then
		touch /tmp/pika-installer-gtk4-swaplist
		echo $PARTITION >  /tmp/pika-installer-gtk4-swaplist
  elif [[ -z $MOUNTOPT ]]
	then
		mkdir -p $MOUNTPOINT
		mount $PARTITION $MOUNTPOINT
	else
		mkdir -p $MOUNTPOINT
		mount -o $MOUNTOPT $PARTITION $MOUNTPOINT
  fi
done

if [[ ! -f "/tmp/pika-installer-gtk4-crypttab" ]]
then
     pikainstall -r /media/pika-install-mount/ --manual 1 -l ${LOCALE} -k ${KEYBOARD} -t ${TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt || touch /tmp/pika-installer-gtk4-fail.txt && exit 1
else
    pikainstall -r /media/pika-install-mount/ --manual 2 -l ${LOCALE} -k ${KEYBOARD} -t ${TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt || touch /tmp/pika-installer-gtk4-fail.txt && exit 1
fi
