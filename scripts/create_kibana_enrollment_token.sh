#!/bin/sh

ANSIBLE_CONFIG="$ANSIBLE_CFG" \
    ansible-playbook \
    -i "$INVENTORY_FILE" \
    "$ANSIBLE_PROVISON_PATH/app/deploy.yml"
