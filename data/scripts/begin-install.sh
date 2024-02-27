#! /bin/bash

exec &> >(tee /tmp/pika-installer-gtk4-log)

if [[ -f /tmp/pika-installer-gtk4-target-manual.txt ]]
then
    sudo /usr/lib/pika/pika-installer-gtk4/scripts/manual-partition-install.sh || sudo touch /tmp/pika-installer-gtk4-fail.txt
else
    if [[ -f /tmp/pika-installer-gtk4-target-auto.txt ]]
    then
        sudo /usr/lib/pika/pika-installer-gtk4/scripts/automatic-partition-install.sh || sudo touch /tmp/pika-installer-gtk4-fail.txt
    else
        echo "critical installer error" && sudo touch /tmp/pika-installer-gtk4-fail.txt && exit 1
    fi
fi
