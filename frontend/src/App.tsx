import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { Bot, Zap, TrendingUp } from 'lucide-react'
import MarketList from './components/MarketList'
import AIChat from './components/AIChat'
import Stats from './components/Stats'
import LiveFeed from './components/LiveFeed'
import { useLinera } from './context/LineraContext'

const queryClient = new QueryClient()

function AppContent() {
  const { connect, isConnected } = useLinera();

  return (
    <div className="min-h-screen bg-dark-bg text-white font-sans">
      {/* Header */}
      <header className="border-b border-neon-blue/20 glass sticky top-0 z-50 backdrop-blur-md">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-lg bg-gradient-to-br from-neon-blue to-neon-purple flex items-center justify-center animate-glow overflow-hidden">
                <img src="/logo.png" alt="OracleAI Logo" className="w-full h-full object-cover" />
              </div>
              <div>
                <h1 className="text-2xl font-bold text-gradient tracking-tight">
                  OracleAI
                </h1>
                <p className="text-xs text-gray-400 font-mono">AI-Powered Prediction Layer</p>
              </div>
            </div>

            <div className="flex items-center gap-6">
              <div className="hidden md:flex items-center gap-2 px-4 py-2 glass rounded-lg border border-neon-purple/30">
                <Zap className="w-4 h-4 text-yellow-400" />
                <span className="font-mono text-sm text-neon-blue">1,234 ORACLE</span>
              </div>
              
              <button 
                onClick={connect}
                className={`px-6 py-2 rounded-lg font-semibold transition-all duration-300 border ${
                  isConnected 
                    ? 'bg-green-500/10 text-green-400 border-green-500/50'
                    : 'bg-gradient-to-r from-neon-blue/10 to-neon-purple/10 border-neon-blue/50 text-neon-blue hover:bg-neon-blue/20 hover:shadow-[0_0_15px_rgba(0,240,255,0.3)]'
                }`}
              >
                {isConnected ? 'Connected' : 'Connect Wallet'}
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="container mx-auto px-4 py-8">
        {/* Stats Banner */}
        <Stats />

        <div className="grid grid-cols-1 lg:grid-cols-4 gap-6 mt-8">
          {/* AI Chat - Left */}
          <div className="lg:col-span-1 h-[600px]">
            <AIChat />
          </div>

          {/* Markets - Center */}
          <div className="lg:col-span-2 h-[600px] overflow-y-auto no-scrollbar">
            <MarketList />
          </div>

          {/* Live Feed - Right */}
          <div className="lg:col-span-1 h-[600px]">
            <LiveFeed />
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="border-t border-white/5 mt-20 py-8 bg-black/20">
        <div className="container mx-auto px-4 text-center text-gray-500 text-xs font-mono">
          <p>🤖 OracleAI • Built on Linera Microchains</p>
          <p className="mt-2 opacity-50">Wave 3 Buildathon 2025</p>
        </div>
      </footer>
    </div>
  )
}

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <AppContent />
    </QueryClientProvider>
  )
}

export default App
