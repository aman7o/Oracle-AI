# 🤖 OracleAI - AI-Powered Autonomous Prediction Markets

**The first prediction market platform where AI agents autonomously create, price, and resolve markets in real-time.**

Built on Linera blockchain • Wave 3 Buildathon Submission • 2025

![OracleAI Banner](./frontend/public/logo.png)

---

## 🌟 **What Makes This Special?**

OracleAI is **NOT just another prediction market**. It's the first platform that leverages:

✨ **AI Agents Create Markets** - Ask AI: "Will it rain tomorrow?" → Market created in 2 seconds
🔮 **AI Oracle Resolution** - Multi-source verification using Claude API
⚡ **Real-Time Trading** - Sub-50ms updates (impossible on Ethereum!)
🤖 **Autonomous Market Makers** - AI agents provide liquidity automatically
🔒 **Zero Front-Running** - MCP/GraphQL integration = no centralized RPC
🎨 **Future-Fi UI** - Cyberpunk Bloomberg-from-2077 interface

---

## 🏗️ **Architecture**

### **4-Application System**

```
┌─────────────────────────────────────────────────────┐
│  AI Layer (Python + Claude API)                     │
│  - Market Creation Bot                              │
│  - Oracle Resolution Service                         │
│  - Trading Agents                                   │
└─────────────────────────────────────────────────────┘
                    │ MCP/GraphQL
                    ▼
┌─────────────────────────────────────────────────────┐
│  Linera Smart Contracts (Rust + WASM)               │
│  ┌─────────┬─────────┬─────────┬───────────┐      │
│  │ Token   │ Market  │ Oracle  │ AI-Agent  │      │
│  │ App     │ App     │ App     │ App       │      │
│  └─────────┴─────────┴─────────┴───────────┘      │
└─────────────────────────────────────────────────────┘
                    │ GraphQL
                    ▼
┌─────────────────────────────────────────────────────┐
│  Frontend (React + TypeScript + TailwindCSS)        │
│  - Cyberpunk "Future-Fi" Design                     │
│  - Real-Time Market Feed                            │
│  - AI Activity Dashboard                            │
│  - WebGL Shader Effects                             │
└─────────────────────────────────────────────────────┘
```

### **Application Details**

#### 1. **TOKEN APP** ✅
- User balance management
- Daily bonus system (100 tokens/24h)
- Transfer operations
- Cross-app balance queries

#### 2. **MARKET APP** ✅
- Market creation & lifecycle
- Bet placement
- Payout calculation
- Event streaming for real-time updates

#### 3. **ORACLE APP** ✅
- Claude API integration
- Multi-source data fetching
- Confidence scoring
- Decentralized oracle on personal chains

#### 4. **AI-AGENT APP** ✅
- Market maker bots
- Strategy execution
- Performance tracking
- Leaderboard

---

## 🚀 **Quick Start**

### **Prerequisites**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# Install Linera CLI (v0.15.6)
cargo install linera-service linera-sdk

# Install Node.js (v18+)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install Python (3.10+)
sudo apt-get install python3 python3-pip
```

### **Run the Project**

1.  **Start Linera Network**
    ```bash
    linera net up
    linera service --port 8080
    ```

2.  **Run AI Oracle Service**
    ```bash
    cd ai-oracle
    pip install -r requirements.txt
    python oracle_service.py
    ```

3.  **Run Frontend**
    ```bash
    cd frontend
    npm install
    npm run dev
    ```

---

## 📁 **Project Structure**

```
oracle-ai/
├── Cargo.toml                 # Workspace configuration
├── rust-toolchain.toml        # Rust 1.86.0
│
├── abi/                       # Shared types
│   └── src/lib.rs             # Market, Bet, AI types
│
├── token/                     # ✅ Token App
│   └── src/
│       ├── lib.rs             # ABI & operations
│       ├── state.rs           # Balance state
│       ├── contract.rs        # Business logic
│       └── service.rs         # GraphQL queries
│
├── market/                    # ✅ Market App
│   └── src/
│       ├── lib.rs             # Market operations
│       ├── state.rs           # Market & bet state
│       ├── contract.rs        # Market logic
│       └── service.rs         # Market queries
│
├── oracle/                    # ✅ Oracle App
│   └── src/
│       ├── lib.rs             # Oracle operations
│       ├── state.rs           # Resolution state
│       ├── contract.rs        # AI resolution logic
│       └── service.rs         # Oracle queries
│
├── ai-agent/                  # ✅ AI-Agent App
│   └── src/
│       ├── lib.rs             # Agent operations
│       ├── state.rs           # Agent state
│       ├── contract.rs        # Trading logic
│       └── service.rs         # Agent queries
│
├── ai-oracle/                 # ✅ Python AI Service
│   ├── oracle_service.py      # Main oracle service
│   └── requirements.txt       # Python deps
│
├── frontend/                  # ✅ React Frontend
│   ├── src/
│   │   ├── components/        # UI components
│   │   ├── context/           # Linera Context
│   │   └── App.tsx            # Main app
│   ├── package.json
│   └── tailwind.config.js
│
└── README.md                  # This file
```

---


---

## 📄 **License**

MIT License - Build the future of autonomous markets!

---

**🤖 OracleAI - Where AI Meets Prediction Markets**

• Powered by Linera • Wave 3 Buildathon 2025*

---

