# AI Oracle Service

Python service that monitors prediction markets and resolves them using Claude API.

## Setup

```bash
# Install dependencies
pip install -r requirements.txt

# Set environment variables
export ANTHROPIC_API_KEY="your-key-here"
export GRAPHQL_ENDPOINT="http://localhost:8080/graphql"

# Run service
python oracle_service.py
```

## How It Works

1. **Monitor Markets**: Checks GraphQL every 10 seconds for closed markets
2. **Gather Data**: Fetches real-world data (prices, weather, etc.)
3. **Ask Claude**: Sends question + data to Claude for analysis
4. **Submit Resolution**: Posts outcome back to blockchain

## Features

- ✅ Claude 3.5 Sonnet integration
- ✅ Multi-source data verification
- ✅ Confidence scoring
- ✅ Automatic market resolution
- ✅ Error handling and retry logic
