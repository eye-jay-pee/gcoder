#!/bin/bash
set -euo pipefail

[[ $# -eq 1 ]] || { echo "Usage: $(basename "$0") <device_path>" >&2; exit 1; }
[[ -e "$1" ]] || { echo "$(basename "$0"): "$1" not found"; exit 1; }

dev_path=$1                                             # ex: /dev/ttyACM0
dev_name=$(basename "$dev_path")                        # ex: ttyACM0
dev_id=$(readlink -f "/sys/class/tty/$dev_name/device") # ex: 1-1.3:1.0
dev_id=$(basename "$dev_id")



if ! printf '%s' "$dev_id" |\
    sudo tee /sys/bus/usb/drivers/cdc_acm/unbind >/dev/null; then 
    echo "failed to unbind $dev_id - maybe they weren't bound?..." >&2 
    # don't exit in this case, since if the driver was already unbound,
    # this step will have failed, but proceeding as usual will handle that. 
else
    echo "$dev_id unbound from $dev_path"
fi
sleep 0.1
if ! printf '%s' "$dev_id" |\
    sudo tee /sys/bus/usb/drivers/cdc_acm/bind   >/dev/null; then 
    echo "failed to (re)bind $dev_id" >&2; exit 1
else 
    echo "$dev_id bound to $dev_path"
fi

