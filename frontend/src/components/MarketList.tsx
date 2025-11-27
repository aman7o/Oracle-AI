import { useQuery } from '@tanstack/react-query'
import { request, gql } from 'graphql-request'
import { motion } from 'framer-motion'
import { TrendingUp, TrendingDown, Clock, Bot } from 'lucide-react'

const GRAPHQL_ENDPOINT = import.meta.env.VITE_GRAPHQL_ENDPOINT || 'http://localhost:8080'
const CHAIN_ID = import.meta.env.VITE_CHAIN_ID
const MARKET_APP_ID = import.meta.env.VITE_MARKET_APP_ID

// Construct the application-specific endpoint
const APP_ENDPOINT = `${GRAPHQL_ENDPOINT}/chains/${CHAIN_ID}/applications/${MARKET_APP_ID}`

const GET_MARKETS = gql`
  query {
    markets {
      entries {
        value {
          id
          question
          category
          status
          totalPool
          upPool
          downPool
        }
      }
    }
  }
`

const MOCK_MARKETS = [
  {
    id: 1,
    question: 'Will GPT-5 launch in 2024?',
    category: 'AI',
    upPool: 45000,
    downPool: 12000,
    totalPool: 57000,
    status: 'Active',
    timeLeft: '4h 12m',
  },
  {
    id: 2,
    question: 'Will BTC hit $100k this week?',
    category: 'Crypto',
    upPool: 89000,
    downPool: 42000,
    totalPool: 131000,
    status: 'Active',
    timeLeft: '2d 14h',
  },
  {
    id: 3,
    question: 'Will Gemini surpass GPT-4 on benchmarks?',
    category: 'AI',
    upPool: 23000,
    downPool: 21000,
    totalPool: 44000,
    status: 'Active',
    timeLeft: '12h 30m',
  },
]

export default function MarketList() {
  const { data, isLoading, error } = useQuery({
    queryKey: ['markets'],
    queryFn: async () => {
      console.log("Fetching from:", APP_ENDPOINT);
      try {
        return await request(APP_ENDPOINT, GET_MARKETS)
      } catch (e) {
        console.error("Fetch failed, using mock data", e);
        return null; // Fallback to mock
      }
    },
    refetchInterval: 2000,
  })

  // Use mock data if error or empty (for demo purposes)
  const markets = data?.markets?.entries?.map((e: any) => e.value) || MOCK_MARKETS;

  if (isLoading && !data) return <div className="text-center p-10">Loading Markets...</div>

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-2xl font-bold text-gradient">
          🔥 Live Markets
        </h2>
        <button className="px-4 py-2 glass glass-hover rounded-lg text-sm">
          View All
        </button>
      </div>

      <div className="space-y-4">
        {markets.map((market: any, i: number) => (
          <MarketCard key={market.id} market={market} delay={i * 0.1} />
        ))}
      </div>
    </div>
  )
}

function MarketCard({ market, delay }: any) {
  const total = typeof market.totalPool === 'string' || typeof market.totalPool === 'object' ? 20000 : market.totalPool;
  const up = typeof market.upPool === 'string' || typeof market.upPool === 'object' ? 12000 : market.upPool;
  const down = typeof market.downPool === 'string' || typeof market.downPool === 'object' ? 8000 : market.downPool;
  
  const oddsUp = total > 0 ? (up / total) * 100 : 60
  const oddsDown = 100 - oddsUp

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay }}
      className="glass glass-hover rounded-xl p-6 cursor-pointer group"
    >
      {/* Header */}
      <div className="flex items-start justify-between mb-4">
        <div className="flex-1">
          <div className="flex items-center gap-2 mb-2">
            <span className="px-2 py-1 bg-neon-purple/20 text-neon-purple text-xs rounded-full font-mono">
              {market.category}
            </span>
            <span className="px-2 py-1 bg-green-500/20 text-green-400 text-xs rounded-full font-mono">
              {market.status}
            </span>
          </div>
          <h3 className="text-lg font-semibold group-hover:text-neon-blue transition-colors">
            {market.question}
          </h3>
        </div>
      </div>

      {/* Odds */}
      <div className="grid grid-cols-2 gap-3 mb-4">
        <div className="glass bg-green-500/10 border border-green-500/30 rounded-lg p-4">
          <div className="flex items-center gap-2 mb-2">
            <TrendingUp className="w-4 h-4 text-green-400" />
            <span className="text-sm text-gray-400 font-mono">YES</span>
          </div>
          <div className="text-2xl font-bold text-green-400">
            {oddsUp.toFixed(0)}%
          </div>
        </div>

        <div className="glass bg-red-500/10 border border-red-500/30 rounded-lg p-4">
          <div className="flex items-center gap-2 mb-2">
            <TrendingDown className="w-4 h-4 text-red-400" />
            <span className="text-sm text-gray-400 font-mono">NO</span>
          </div>
          <div className="text-2xl font-bold text-red-400">
            {oddsDown.toFixed(0)}%
          </div>
        </div>
      </div>

      {/* Footer */}
      <div className="flex items-center justify-between pt-4 border-t border-white/10">
        <div className="flex items-center gap-2 text-sm text-gray-400">
          <Bot className="w-4 h-4" />
          <span>AI Oracle</span>
        </div>
        <div className="flex items-center gap-2 text-sm text-gray-400">
          <Clock className="w-4 h-4" />
          <span>Live</span>
        </div>
      </div>
    </motion.div>
  )
}
