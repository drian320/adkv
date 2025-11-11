#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"
sudo -E LD_LIBRARY_PATH=./memflow_lib/target/release:$LD_LIBRARY_PATH ./build/apex_dma "$@"
