#!/bin/sh
set -eu

# this is persistant, but not between rebuilds

DEBUG_TARG="target/debug/gcoder"
RELEASE_TARG="target/release/gcoder"

for f in $DEBUG_TARG $RELEASE_TARG; do 
    if [ -e "$f" ]; then 
        sudo chown root:root "$f"
        sudo chmod u+s "$f"
    fi
done
