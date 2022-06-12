#!/bin/sh

if [[ "$1" == "fast" ]];
then
    skip_tags="build_full"
else
    skip_tags=""
fi

ANSIBLE_CONFIG="$ANSIBLE_CFG" \
    ansible-playbook \
    -i "$INVENTORY_FILE" \
    --skip-tags "$skip_tags" \
    "$ANSIBLE_PROVISON_PATH/app/deploy.yml"
