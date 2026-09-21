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

# Release archives don't extract to a consistent depth across platforms (the
# mac-universal asset nests its payload one directory deeper than
# `--strip-components=1` accounts for; Linux assets may nest differently), so
# resolve the binary by searching rather than assuming a fixed path
# (RESEARCH.md Pitfall 3).
resolve_bin() {
    find "$NODE_DIR" -type f -name ink-node 2>/dev/null | head -n 1
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
}

RESOLVE_ONLY=0
if [ "${1:-}" = "--resolve-only" ]; then
    RESOLVE_ONLY=1
    shift
fi

BIN_PATH="$(resolve_bin)"

# Only a genuinely missing binary triggers a (re-)download. A binary that
# resolves but lacks the executable bit (e.g. permissions not preserved by an
# extraction, a filesystem copy, or the underlying volume) is a local
# permissions issue fixed by chmod below, not a reason to re-fetch a ~300MB
# release asset.
if [ -z "$BIN_PATH" ]; then
    if [ "$RESOLVE_ONLY" -eq 1 ]; then
        echo "ink-node binary not found under $NODE_DIR" >&2
        exit 1
    fi
    install_ink_node
    BIN_PATH="$(resolve_bin)"
    if [ -z "$BIN_PATH" ]; then
        echo "ink-node binary still not found under $NODE_DIR after install" >&2
        exit 1
    fi
fi

chmod +x "$BIN_PATH"

if [ "$(uname -s)" = "Darwin" ]; then
    xattr -dr com.apple.quarantine "$BIN_PATH" 2>/dev/null || true
fi

if [ ! -x "$BIN_PATH" ]; then
    echo "ink-node binary at $BIN_PATH could not be made executable" >&2
    exit 1
fi

if [ "$RESOLVE_ONLY" -eq 1 ]; then
    echo "ink-node binary: $BIN_PATH"
    exit 0
fi

echo "Starting ink-node (--dev --tmp)..."
exec "$BIN_PATH" --dev --tmp "$@"
