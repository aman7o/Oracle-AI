import { useState } from 'react'
import { Bot, Send, Sparkles } from 'lucide-react'
import { motion } from 'framer-motion'

export default function AIChat() {
  const [input, setInput] = useState('')
  const [messages, setMessages] = useState([
    {
      role: 'ai',
      text: 'Hi! I can create prediction markets for you. Just describe what you want to bet on!',
    },
  ])

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (!input.trim()) return

    // Add user message
    setMessages([...messages, { role: 'user', text: input }])

    // Simulate AI response
    setTimeout(() => {
      setMessages((prev) => [
        ...prev,
        {
          role: 'ai',
          text: `Creating market: "${input}"\n\n✅ Market created!\n📊 Initial odds: 50/50\n🤖 Oracle: AI-powered\n⏰ Duration: 24 hours`,
        },
      ])
    }, 1000)

    setInput('')
  }

  return (
    <div className="glass rounded-xl p-6 h-[600px] flex flex-col">
      <div className="flex items-center gap-3 mb-4 pb-4 border-b border-white/10">
        <div className="w-10 h-10 rounded-lg bg-gradient-to-br from-neon-purple to-neon-pink flex items-center justify-center">
          <Sparkles className="w-5 h-5 text-white" />
        </div>
        <div>
          <h2 className="font-bold text-lg">AI Market Creator</h2>
          <p className="text-xs text-gray-400">Ask AI to create markets</p>
        </div>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto space-y-4 mb-4">
        {messages.map((msg, i) => (
          <motion.div
            key={i}
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            className={`flex gap-3 ${msg.role === 'user' ? 'justify-end' : ''}`}
          >
            {msg.role === 'ai' && (
              <div className="w-8 h-8 rounded-lg bg-neon-purple/20 flex items-center justify-center flex-shrink-0">
                <Bot className="w-4 h-4 text-neon-purple" />
              </div>
            )}
            <div
              className={`px-4 py-2 rounded-lg max-w-[80%] ${
                msg.role === 'user'
                  ? 'bg-neon-blue/20 text-white'
                  : 'glass'
              }`}
            >
              <p className="text-sm whitespace-pre-line">{msg.text}</p>
            </div>
          </motion.div>
        ))}
      </div>

      {/* Input */}
      <form onSubmit={handleSubmit} className="flex gap-2">
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder="Will BTC hit $100k this week?"
          className="flex-1 px-4 py-3 glass rounded-lg focus:outline-none focus:glow-border bg-transparent"
        />
        <button
          type="submit"
          className="px-4 py-3 bg-gradient-to-r from-neon-blue to-neon-purple rounded-lg hover:shadow-lg hover:shadow-neon-blue/50 transition-all"
        >
          <Send className="w-5 h-5" />
        </button>
      </form>
    </div>
  )
}
