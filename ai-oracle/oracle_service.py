#!/usr/bin/env python3
"""
OracleAI - AI Oracle Service

Monitors prediction markets and resolves them using Claude API.
"""

import asyncio
import os
import json
from typing import Dict, List, Optional
from anthropic import Anthropic
from gql import gql, Client
from gql.transport.aiohttp import AIOHTTPTransport
import aiohttp


class AIOracle:
    """AI-powered oracle for prediction markets"""

    def __init__(self, graphql_endpoint: str, anthropic_key: str):
        """Initialize the oracle"""
        # GraphQL client
        transport = AIOHTTPTransport(url=graphql_endpoint)
        self.gql_client = Client(
            transport=transport,
            fetch_schema_from_transport=True
        )

        # Anthropic Claude client
        self.anthropic = Anthropic(api_key=anthropic_key)

        print("🤖 OracleAI Service initialized")
        print(f"📡 GraphQL endpoint: {graphql_endpoint}")
        print(f"🔑 Claude API key: {'✓' if anthropic_key else '✗'}")

    async def run(self):
        """Main loop - monitor and resolve markets"""
        print("\n🚀 Starting oracle service...")
        print("⏰ Checking for markets every 10 seconds\n")

        while True:
            try:
                await self.process_markets()
                await asyncio.sleep(10)
            except KeyboardInterrupt:
                print("\n👋 Shutting down oracle service...")
                break
            except Exception as e:
                print(f"❌ Error: {e}")
                await asyncio.sleep(30)

    async def process_markets(self):
        """Check for markets that need resolution"""
        try:
            markets = await self.fetch_markets()

            for market in markets:
                status = market.get('status', 'Unknown')

                if status == 'Active':
                    # Check if market should close
                    # For demo, we'll auto-resolve after some time
                    pass
                elif status == 'Closed':
                    # Market is closed, needs resolution
                    await self.resolve_market(market)

        except Exception as e:
            print(f"⚠️  Error processing markets: {e}")

    async def fetch_markets(self) -> List[Dict]:
        """Fetch all markets from blockchain"""
        try:
            query = gql("""
                query {
                    markets {
                        entries {
                            key
                            value {
                                id
                                question
                                description
                                category
                                status
                                oracleMode
                                totalPool
                                upPool
                                downPool
                            }
                        }
                    }
                }
            """)

            result = await self.gql_client.execute_async(query)
            entries = result.get('markets', {}).get('entries', [])
            return [entry['value'] for entry in entries]

        except Exception as e:
            print(f"⚠️  Error fetching markets: {e}")
            return []

    async def resolve_market(self, market: Dict):
        """Resolve a market using AI"""
        market_id = market['id']
        question = market['question']
        category = market.get('category', 'Custom')

        print(f"\n🔍 Resolving Market #{market_id}")
        print(f"   Question: {question}")
        print(f"   Category: {category}")

        # Gather data
        print("   📊 Gathering data...")
        data_sources = await self.fetch_verification_data(category, question)

        # Ask Claude
        print("   🤖 Consulting Claude AI...")
        analysis = await self.ask_claude(question, data_sources)

        # Submit resolution
        print(f"   ✅ Outcome: {analysis['outcome']}")
        print(f"   📈 Confidence: {analysis['confidence']}%")
        print(f"   💭 Reasoning: {analysis['reasoning']}")

        await self.submit_resolution(
            market_id,
            analysis['outcome'],
            analysis['confidence'],
            analysis['reasoning'],
            analysis['sources']
        )

        print(f"   🎉 Market #{market_id} resolved!\n")

    async def ask_claude(self, question: str, data_sources: List[Dict]) -> Dict:
        """Query Claude API for market analysis"""

        # Format data sources
        sources_text = "\n".join([
            f"- {s['name']}: {s['data']}"
            for s in data_sources
        ])

        prompt = f"""You are an oracle for a prediction market platform called OracleAI.

Analyze the following question and data sources to determine the outcome.

Question: {question}

Data Sources:
{sources_text if sources_text else "No external data available"}

Instructions:
1. Determine if the outcome is "UP" (YES) or "DOWN" (NO)
2. Provide a confidence score from 0-100
3. Explain your reasoning in 2-3 sentences
4. List the sources you used

Respond in JSON format:
{{
    "outcome": "UP" or "DOWN",
    "confidence": 95,
    "reasoning": "explanation here",
    "sources": ["url1", "url2"]
}}

Be objective and data-driven. If data is insufficient, use lower confidence.
"""

        try:
            message = self.anthropic.messages.create(
                model="claude-3-5-sonnet-20241022",
                max_tokens=1024,
                messages=[{
                    "role": "user",
                    "content": prompt
                }]
            )

            response_text = message.content[0].text

            # Parse JSON response
            # Claude sometimes wraps JSON in markdown code blocks
            if "```json" in response_text:
                response_text = response_text.split("```json")[1].split("```")[0]
            elif "```" in response_text:
                response_text = response_text.split("```")[1].split("```")[0]

            analysis = json.loads(response_text.strip())

            # Validate response
            if analysis.get('outcome') not in ['UP', 'DOWN']:
                analysis['outcome'] = 'UP'  # Default
            if not isinstance(analysis.get('confidence'), (int, float)):
                analysis['confidence'] = 50

            return analysis

        except Exception as e:
            print(f"⚠️  Claude API error: {e}")
            # Return default response
            return {
                "outcome": "UP",
                "confidence": 50,
                "reasoning": "Unable to determine outcome due to API error",
                "sources": []
            }

    async def fetch_verification_data(
        self,
        category: str,
        question: str
    ) -> List[Dict]:
        """Fetch real-world data for verification"""
        sources = []

        try:
            async with aiohttp.ClientSession() as session:
                # Crypto prices
                if 'BTC' in question or 'crypto' in category.lower():
                    try:
                        async with session.get(
                            'https://api.coinbase.com/v2/prices/BTC-USD/spot'
                        ) as resp:
                            if resp.status == 200:
                                data = await resp.json()
                                sources.append({
                                    'name': 'Coinbase',
                                    'url': 'https://api.coinbase.com',
                                    'data': f"BTC Price: ${data['data']['amount']}"
                                })
                    except:
                        pass

                # Weather (example - would need API key)
                if 'weather' in question.lower() or 'rain' in question.lower():
                    sources.append({
                        'name': 'Weather API',
                        'url': 'https://weather.com',
                        'data': 'Weather data (demo mode)'
                    })

                # Add more data sources as needed

        except Exception as e:
            print(f"⚠️  Error fetching verification data: {e}")

        return sources

    async def submit_resolution(
        self,
        market_id: int,
        outcome: str,
        confidence: float,
        reasoning: str,
        sources: List[str]
    ):
        """Submit resolution to blockchain"""
        try:
            # Convert outcome to enum
            outcome_enum = "Up" if outcome == "UP" else "Down"

            mutation = gql(f"""
                mutation {{
                    resolveMarketAI(
                        marketId: {market_id},
                        outcome: {outcome_enum},
                        confidence: {confidence},
                        reasoning: "{reasoning}",
                        sources: {json.dumps(sources)}
                    )
                }}
            """)

            await self.gql_client.execute_async(mutation)

        except Exception as e:
            print(f"⚠️  Error submitting resolution: {e}")


async def main():
    """Main entry point"""
    # Get configuration from environment
    graphql_endpoint = os.getenv(
        'GRAPHQL_ENDPOINT',
        'http://localhost:8080/graphql'
    )
    anthropic_key = os.getenv('ANTHROPIC_API_KEY')

    if not anthropic_key:
        print("❌ ERROR: ANTHROPIC_API_KEY environment variable not set")
        print("Get your key from: https://console.anthropic.com/")
        return

    # Create and run oracle
    oracle = AIOracle(graphql_endpoint, anthropic_key)
    await oracle.run()


if __name__ == "__main__":
    asyncio.run(main())
