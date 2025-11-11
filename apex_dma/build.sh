#!/bin/bash

cd memflow_lib/memflow-ffi/
if cargo build --release ; then
    cd ../../
    make

    # 実行用スクリプトを生成
    echo "Generating run script..."
    cat > run_apex_dma.sh << 'EOF'
#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"
sudo -E LD_LIBRARY_PATH=./memflow_lib/target/release:$LD_LIBRARY_PATH ./build/apex_dma "$@"
EOF
    chmod +x run_apex_dma.sh
    echo "Build complete! Run with: ./run_apex_dma.sh"
else
    echo "Error while building memflow-ffi"
fi
