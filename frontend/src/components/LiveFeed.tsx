import { useEffect, useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { Terminal, Activity, CheckCircle } from 'lucide-react'

const LOGS = [
  { type: 'info', msg: 'Agent-007 scanning market "GPT-5 Launch"...' },
  { type: 'process', msg: 'Fetching data from Anthropic API...' },
  { type: 'success', msg: 'Analysis Complete: Bullish (87% Confidence)' },
  { type: 'tx', msg: 'Placing Bet: 500 ORACLE on YES' },
  { type: 'confirm', msg: 'Transaction Confirmed (Block #452)' },
  { type: 'info', msg: 'Agent-X scanning market "BTC $100k"...' },
  { type: 'process', msg: 'Analyzing Twitter sentiment...' },
  { type: 'error', msg: 'Sentiment Analysis: Mixed/Neutral' },
  { type: 'tx', msg: 'Strategy: HOLD' },
]

export default function LiveFeed() {
  const [logs, setLogs] = useState<any[]>([])

  useEffect(() => {
    let i = 0
    const interval = setInterval(() => {
      setLogs(prev => [LOGS[i % LOGS.length], ...prev].slice(0, 6))
      i++
    }, 2500)
    return () => clearInterval(interval)
  }, [])

  return (
    <div className="glass rounded-xl p-4 h-full border border-neon-blue/20 flex flex-col">
      <div className="flex items-center gap-2 mb-4 text-neon-blue">
        <Terminal className="w-4 h-4" />
        <h3 className="text-sm font-mono font-bold uppercase tracking-wider">Agent Activity Log</h3>
        <div className="ml-auto flex items-center gap-1">
          <span className="w-2 h-2 rounded-full bg-green-500 animate-pulse"></span>
          <span className="text-xs text-green-500">ONLINE</span>
        </div>
      </div>

      <div className="flex-1 overflow-hidden relative space-y-3">
        <AnimatePresence initial={false}>
          {logs.map((log, i) => (
            <motion.div
              key={i}
              initial={{ opacity: 0, x: -20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, y: 10 }}
              className="flex items-start gap-3 text-xs font-mono"
            >
              <span className="text-gray-500">
                [{new Date().toLocaleTimeString([], { hour12: false })}]
              </span>
              <span className={`
                ${log.type === 'success' ? 'text-green-400' : ''}
                ${log.type === 'confirm' ? 'text-neon-purple' : ''}
                ${log.type === 'tx' ? 'text-yellow-400' : ''}
                ${log.type === 'error' ? 'text-red-400' : ''}
                ${log.type === 'process' ? 'text-blue-400' : ''}
                ${log.type === 'info' ? 'text-gray-300' : ''}
              `}>
                {log.msg}
              </span>
            </motion.div>
          ))}
        </AnimatePresence>
        
        {/* Gradient Fade at Bottom */}
        <div className="absolute bottom-0 left-0 right-0 h-12 bg-gradient-to-t from-dark-card to-transparent pointer-events-none"></div>
      </div>
    </div>
  )
}
