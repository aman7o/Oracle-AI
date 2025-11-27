#!/bin/bash

echo "🚀 Starting OracleAI..."

# Build contracts
echo "📦 Building smart contracts..."
cargo build --release --target wasm32-unknown-unknown

# Start Linera local network
echo "🌐 Starting Linera network..."
linera net up &
sleep 5

# Deploy applications
echo "🚢 Deploying applications..."

# Token app
TOKEN_APP=$(linera project publish-and-create token \
  --json-parameters '{"initial_supply": "1000000000000"}' | grep "New application" | awk '{print $NF}')
echo "✅ Token app: $TOKEN_APP"

# Market app
MARKET_APP=$(linera project publish-and-create market \
  --json-parameters "{\"token_app\": \"$TOKEN_APP\"}" | grep "New application" | awk '{print $NF}')
echo "✅ Market app: $MARKET_APP"

# Oracle app
ORACLE_APP=$(linera project publish-and-create oracle \
  --json-parameters "{\"market_app\": \"$MARKET_APP\"}" | grep "New application" | awk '{print $NF}')
echo "✅ Oracle app: $ORACLE_APP"

# AI-Agent app
AGENT_APP=$(linera project publish-and-create ai-agent \
  --json-parameters "{\"market_app\": \"$MARKET_APP\", \"token_app\": \"$TOKEN_APP\"}" | grep "New application" | awk '{print $NF}')
echo "✅ AI-Agent app: $AGENT_APP"

# Start GraphQL service
echo "🔗 Starting GraphQL service..."
linera service --port 8080 &

# Start AI Oracle service
echo "🤖 Starting AI Oracle service..."
cd ai-oracle
python3 oracle_service.py &
cd ..

# Start frontend
echo "🎨 Starting frontend..."
cd frontend
npm run dev -- --host 0.0.0.0 &
cd ..

echo ""
echo "✅ OracleAI is running!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🌐 Frontend: http://localhost:5173"
echo "🔗 GraphQL: http://localhost:8080/graphql"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "App IDs:"
echo "  Token:    $TOKEN_APP"
echo "  Market:   $MARKET_APP"
echo "  Oracle:   $ORACLE_APP"
echo "  AI-Agent: $AGENT_APP"
echo ""

# Keep container running
tail -f /dev/null
