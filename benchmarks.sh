#!/usr/bin/env bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
folder_path="test_files/"
if [ ! -d "$SCRIPT_DIR/$folder_path" ]; then
    mkdir -p "$SCRIPT_DIR/$folder_path"
fi
cd "$SCRIPT_DIR"/$folder_path || exit
rm ** -f ;touch Episode\ {50000..1}.mp4
# rm -rf $SCRIPT_DIR/test || true
