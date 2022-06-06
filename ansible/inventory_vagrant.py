#!/bin/env python3

import re
import sys
import argparse
import json

PAT = re.compile(
    r"config\.vm\.define\s*\"([a-z0-9_.]+)\".*?app\.vm\.network.*?ip:\s*\"([0-9.]+)",
    re.MULTILINE | re.DOTALL | re.IGNORECASE,
)


def parse_vagrantfile(file_path: str):
    with open(file_path, "r") as f:
        data = f.read()
    groups = {}
    for match in PAT.finditer(data):
        group, ip = match.groups()
        groups[group] = groups.get(group, []) + [ip]
    return groups


def get_group_variables():
    return {
        "ansible_user": "vagrant",
        # "ansible_ssh_private_key_file": "{{ lookup('env', 'ANSIBLE_SSH_KEY_FILE') }}",
        # "ansible_ssh_port": "22",
    }


def create_inventory(file_path: str):
    vagrant_inventory = parse_vagrantfile(file_path)
    inventory = {
        "all": {"hosts": [], "vars": get_group_variables()},
        "_meta": {"hostvars": {}},
    }

    for (group, hosts) in vagrant_inventory.items():
        if group not in inventory:
            inventory[group] = {"hosts": [], "vars": {}}
        for host in hosts:
            if host not in inventory[group]["hosts"]:
                inventory[group]["hosts"].append(host)
            if host not in inventory["all"]["hosts"]:
                inventory["all"]["hosts"].append(host)

    return inventory


def parse_args():
    parser = argparse.ArgumentParser(
        description="Creates dynamic ansible inventory from a Vagrantfile"
    )
    parser.add_argument("--list", action="store_true")
    parser.add_argument(
        "--file",
        metavar="FILE",
        help="The vagrantfile from which to create an ansible inventory",
        default="Vagrantfile",
    )
    return parser.parse_args(sys.argv[1:])


args = parse_args()
inventory = create_inventory(args.file)
print(json.dumps(inventory))
