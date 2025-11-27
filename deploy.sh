#!/bin/bash
# Deployment script for OracleAI

set -e

echo "🚀 OracleAI Deployment Script"
echo "=============================="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found. Please install: https://rustup.rs/"
    exit 1
fi

# Check if Linera CLI is installed
if ! command -v linera &> /dev/null; then
    echo "❌ Linera CLI not found. Please install Linera SDK"
    exit 1
fi

# Build contracts
echo "📦 Building smart contracts..."
cargo build --release --target wasm32-unknown-unknown

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"
echo ""

# Start local network (optional)
read -p "Start local Linera network? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🌐 Starting Linera network..."
    linera net up
    sleep 3
fi

# Deploy applications
echo ""
echo "🚢 Deploying applications..."
echo ""

# Token app
echo "Deploying Token app..."
linera project publish-and-create token \
  --json-parameters '{"initial_supply": "1000000000000"}'

echo ""
read -p "Enter Token app ID: " TOKEN_APP

# Market app
echo ""
echo "Deploying Market app..."
linera project publish-and-create market \
  --json-parameters "{\"token_app\": \"$TOKEN_APP\"}"

echo ""
read -p "Enter Market app ID: " MARKET_APP

# Oracle app
echo ""
echo "Deploying Oracle app..."
linera project publish-and-create oracle \
  --json-parameters "{\"market_app\": \"$MARKET_APP\"}"

echo ""
read -p "Enter Oracle app ID: " ORACLE_APP

# AI-Agent app
echo ""
echo "Deploying AI-Agent app..."
linera project publish-and-create ai-agent \
  --json-parameters "{\"market_app\": \"$MARKET_APP\", \"token_app\": \"$TOKEN_APP\"}"

echo ""
read -p "Enter AI-Agent app ID: " AGENT_APP

# Save app IDs
echo ""
echo "💾 Saving app IDs to .env..."
cat > .env << EOF
# OracleAI Application IDs
TOKEN_APP=$TOKEN_APP
MARKET_APP=$MARKET_APP
ORACLE_APP=$ORACLE_APP
AGENT_APP=$AGENT_APP

# GraphQL Endpoint
VITE_GRAPHQL_ENDPOINT=http://localhost:8080/graphql

# Claude API Key
ANTHROPIC_API_KEY=your-key-here
EOF

echo "✅ Deployment complete!"
echo ""
echo "📝 App IDs saved to .env"
echo ""
echo "Next steps:"
echo "1. Get Claude API key: https://console.anthropic.com/"
echo "2. Update .env with your ANTHROPIC_API_KEY"
echo "3. Start GraphQL service: linera service --port 8080"
echo "4. Start AI Oracle: cd ai-oracle && python3 oracle_service.py"
echo "5. Start Frontend: cd frontend && npm run dev"
echo ""
echo "🎉 Happy building!"
