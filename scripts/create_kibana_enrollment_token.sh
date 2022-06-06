#!/bin/sh

echo "$INVENTORY_FILE"

ANSIBLE_CONFIG="$ANSIBLE_CFG" \
    ansible-playbook \
    -i "$INVENTORY_FILE" \
    --tags enrollment_token \
    "$ELK_PROVSIONING_DIR/main.yml"
