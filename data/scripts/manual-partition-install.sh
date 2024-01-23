#! /bin/bash

set -e

DISK="$(cat "/tmp/pika-installer-gtk4-target-manual.txt")"
LOCALE="$(cat "/tmp/pika-installer-gtk4-lang.txt")"
KEYBOARD="$(cat "/tmp/pika-installer-gtk4-target-auto.txt")"
TIMEZONE="$(cat "/tmp/pika-installer-gtk4-timezone.txt")"

touch "/tmp/pika-installer-gtk4-status-parting.txt"

if [[ ! -f "/tmp/pika-installer-gtk4-target-manual-luks.txt" ]]
then
    pikainstall -r ${DISK}/ -l ${LOCALE} -k ${KEYBOARD} -t ${TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt
else
    LUKS_KEY="$(cat "/tmp/pika-installer-gtk4-target-manual-luks.txt")"
    pikainstall -r ${DISK}/ -c ${LUKS_KEY} -l ${LOCALE} -k ${KEYBOARD} -t ${TIMEZONE} && touch /tmp/pika-installer-gtk4-successful.txt
fi