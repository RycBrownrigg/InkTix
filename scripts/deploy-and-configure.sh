#!/bin/bash
# Build unified contract, deploy to local node, and configure frontend
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
CONTRACT_DIR="$PROJECT_DIR/contracts/inktix"
FRONTEND_DIR="$PROJECT_DIR/frontend"

echo "=== InkTix Local Development Setup ==="

# Step 1: Build the contract
echo ""
echo "--- Building unified contract ---"
cd "$CONTRACT_DIR"
cargo contract build 2>&1

if [ $? -ne 0 ]; then
    echo "Contract build failed!"
    exit 1
fi

echo "Contract built successfully."

# Step 2: Deploy to local node
echo ""
echo "--- Deploying contract to local node ---"
echo "Ensure a local Substrate node is running on ws://127.0.0.1:9944"
echo "You can start one with: docker compose up substrate"
echo ""

# Check if local node is running
if ! curl -s http://127.0.0.1:9944 > /dev/null 2>&1; then
    echo "Local node not running. Start it with:"
    echo "  docker compose up substrate"
    echo ""
    echo "Or manually:"
    echo "  docker run --rm -p 9944:9944 parity/substrate-contracts-node:latest --dev --tmp --rpc-external --rpc-cors=all --unsafe-rpc-external --rpc-methods=unsafe"
    exit 1
fi

# Deploy using cargo-contract (captures the contract address)
DEPLOY_OUTPUT=$(cargo contract instantiate \
    --constructor new \
    --suri //Alice \
    --url ws://127.0.0.1:9944 \
    --skip-confirm \
    2>&1) || true

# Extract contract address from output
CONTRACT_ADDRESS=$(echo "$DEPLOY_OUTPUT" | grep -oP 'Contract [0-9a-zA-Z]+' | head -1 | awk '{print $2}')

if [ -z "$CONTRACT_ADDRESS" ]; then
    echo "Could not extract contract address from deployment output."
    echo "Deployment output:"
    echo "$DEPLOY_OUTPUT"
    echo ""
    echo "You may need to deploy manually and set the address in .env.local"
    exit 0
fi

echo "Contract deployed at: $CONTRACT_ADDRESS"

# Step 3: Configure frontend
echo ""
echo "--- Configuring frontend ---"
cat > "$FRONTEND_DIR/.env.local" << EOF
NEXT_PUBLIC_RPC_ENDPOINT=ws://127.0.0.1:9944
NEXT_PUBLIC_LOCAL_RPC_ENDPOINT=ws://127.0.0.1:9944
NEXT_PUBLIC_CHAIN_NAME=Development
NEXT_PUBLIC_MOCK_MODE=false
NEXT_PUBLIC_CONTRACT_ADDRESS=$CONTRACT_ADDRESS
EOF

echo "Frontend .env.local updated with contract address."
echo ""
echo "=== Setup complete ==="
echo "Start the frontend with: cd frontend && npm run dev"
