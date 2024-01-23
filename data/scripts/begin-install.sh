#! /bin/bash

if [[ -f /tmp/pika-installer-gtk4-target-manual.txt ]]
then
    sudo /usr/lib/pika/pika-installer-gtk4/scripts/manual-partition-install.sh
else
    if [[ -f /tmp/pika-installer-gtk4-target-auto.txt ]]
    then
        sudo /usr/lib/pika/pika-installer-gtk4/scripts/automatic-partition-install.sh
    else
        echo "critical installer error" && exit 1
    fi
fi