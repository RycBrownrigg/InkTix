#!/bin/bash
# Downloads (if missing) and runs ink-node, the local dev chain for ink! v6 /
# pallet-revive contracts. Replaces `docker run parity/substrate-contracts-node`,
# which cannot host pallet-revive contracts. See docs/v2_plan.md Phase 1a.
#
# RPC available at ws://127.0.0.1:9944 once started.

set -euo pipefail

INK_NODE_VERSION="v0.47.0"
TOOLING_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/.tooling"
NODE_DIR="$TOOLING_DIR/ink-node"
BIN_PATH="$NODE_DIR/ink-node"

detect_asset() {
    local os
    local arch
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Darwin)
            echo "ink-node-mac-universal.tar.gz"
            ;;
        Linux)
            case "$arch" in
                aarch64|arm64)
                    echo "ink-node-linux-arm64.tar.gz"
                    ;;
                *)
                    echo "ink-node-linux.tar.gz"
                    ;;
            esac
            ;;
        *)
            echo "Unsupported OS: $os" >&2
            exit 1
            ;;
    esac
}

install_ink_node() {
    local asset
    asset="$(detect_asset)"
    local url="https://github.com/use-ink/ink-node/releases/download/${INK_NODE_VERSION}/${asset}"

    echo "ink-node not found — downloading ${INK_NODE_VERSION} (${asset})..."
    mkdir -p "$NODE_DIR"
    curl -sL -o "$TOOLING_DIR/ink-node.tar.gz" "$url"
    tar xzf "$TOOLING_DIR/ink-node.tar.gz" -C "$NODE_DIR" --strip-components=1
    rm "$TOOLING_DIR/ink-node.tar.gz"
    chmod +x "$BIN_PATH"

    if [ "$(uname -s)" = "Darwin" ]; then
        echo "Clearing macOS quarantine attribute..."
        xattr -dr com.apple.quarantine "$BIN_PATH" 2>/dev/null || true
    fi

    echo "ink-node installed at $BIN_PATH"
}

if [ ! -x "$BIN_PATH" ]; then
    install_ink_node
fi

echo "Starting ink-node (--dev --tmp)..."
exec "$BIN_PATH" --dev --tmp "$@"
