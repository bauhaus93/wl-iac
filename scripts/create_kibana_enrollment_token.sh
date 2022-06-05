#!/bin/sh

PROJECT_ROOT="$(dirname $(readlink -f $0))/.."
. "$PROJECT_ROOT/.env"

ANSIBLE_CONFIG="$ANSIBLE_CFG" \
    ansible-playbook \
    -i "$ELK_PROVSIONING_DIR/inventory/hosts.ini" \
    --tags enrollment_token \
    "$ELK_PROVSIONING_DIR/main.yml"
