import { TrendingUp, Users, DollarSign, Zap } from 'lucide-react'

export default function Stats() {
  return (
    <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
      <StatCard
        icon={<TrendingUp className="w-6 h-6 text-green-400" />}
        label="Active Markets"
        value="24"
        change="+12%"
      />
      <StatCard
        icon={<Users className="w-6 h-6 text-blue-400" />}
        label="AI Agents"
        value="8"
        change="+2"
      />
      <StatCard
        icon={<DollarSign className="w-6 h-6 text-yellow-400" />}
        label="Total Volume"
        value="$142K"
        change="+18%"
      />
      <StatCard
        icon={<Zap className="w-6 h-6 text-purple-400" />}
        label="Avg Resolution"
        value="<50ms"
        change="Real-time"
      />
    </div>
  )
}

function StatCard({ icon, label, value, change }: any) {
  return (
    <div className="glass glass-hover rounded-xl p-4">
      <div className="flex items-center justify-between mb-2">
        <div className="p-2 bg-white/5 rounded-lg">{icon}</div>
        <span className="text-xs text-green-400 font-mono">{change}</span>
      </div>
      <div className="text-2xl font-bold mb-1">{value}</div>
      <div className="text-sm text-gray-400">{label}</div>
    </div>
  )
}
