import React, { createContext, useContext, useState, useEffect, useCallback } from 'react';

// Network Configuration - automatically detects local vs testnet
const isLocalNetwork = typeof window !== 'undefined' &&
  (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1');

// Use environment variables or defaults based on network
const FAUCET_URL = import.meta.env.VITE_LINERA_FAUCET_URL ||
  (isLocalNetwork ? 'http://localhost:8080' : 'https://faucet.testnet-conway.linera.net');

const GRAPHQL_URL = import.meta.env.VITE_LINERA_GRAPHQL_URL ||
  (isLocalNetwork ? 'http://localhost:9001' : 'https://testnet-conway.linera.net/graphql');

// App IDs from environment (set by deploy script)
const APP_IDS = {
  token: import.meta.env.VITE_TOKEN_APP_ID || '',
  market: import.meta.env.VITE_MARKET_APP_ID || '',
  oracle: import.meta.env.VITE_ORACLE_APP_ID || '',
  aiAgent: import.meta.env.VITE_AI_AGENT_APP_ID || '',
};

interface LineraContextType {
  client: any | null;
  chainId: string | null;
  walletAddress: string | null;
  connect: () => Promise<void>;
  disconnect: () => void;
  isConnected: boolean;
  isConnecting: boolean;
  error: string | null;
  appIds: typeof APP_IDS;
  graphqlEndpoint: string;
  faucetUrl: string;
  isLocalNetwork: boolean;
}

const LineraContext = createContext<LineraContextType>({} as LineraContextType);

export function LineraProvider({ children }: { children: React.ReactNode }) {
  const [client, setClient] = useState<any | null>(null);
  const [chainId, setChainId] = useState<string | null>(null);
  const [walletAddress, setWalletAddress] = useState<string | null>(null);
  const [isConnecting, setIsConnecting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Initialize WASM module on mount
  useEffect(() => {
    const initWasm = async () => {
      try {
        const linera = await import('@linera/client');
        await linera.default();
        console.log('Linera WASM initialized');
        console.log('Network:', isLocalNetwork ? 'Local' : 'Conway Testnet');
        console.log('Faucet:', FAUCET_URL);
      } catch (e) {
        console.warn('Linera client not available, using demo mode:', e);
      }
    };
    initWasm();
  }, []);

  const connect = useCallback(async () => {
    setIsConnecting(true);
    setError(null);

    try {
      // Try to use @linera/client for real wallet connection
      const linera = await import('@linera/client');
      await linera.default();

      // Create wallet from faucet
      const faucet = new linera.Faucet(FAUCET_URL);
      const wallet = await faucet.createWallet();
      const lineraClient = new linera.Client(wallet);

      // Claim a chain
      const newChainId = await faucet.claimChain(lineraClient);

      // Get wallet info
      const publicKey = await wallet.getPublicKey();

      setClient(lineraClient);
      setChainId(newChainId);
      setWalletAddress(publicKey);

      console.log('Connected:', {
        chainId: newChainId,
        wallet: publicKey,
        network: isLocalNetwork ? 'local' : 'conway'
      });

    } catch (e: any) {
      console.warn('Wallet connection failed, using demo mode:', e);

      // Fallback to demo mode for presentation
      setClient({
        demo: true,
        executeOperation: async (appId: string, operation: any) => {
          console.log('Demo mode - operation:', { appId, operation });
          return { success: true, demo: true };
        },
        query: async (appId: string, query: string) => {
          console.log('Demo mode - query:', { appId, query });
          return { data: {} };
        }
      });

      // Generate demo identifiers
      setChainId('demo-' + Date.now().toString(16));
      setWalletAddress('demo-wallet-' + Math.random().toString(16).slice(2, 10));
      setError('Demo mode: Wallet not connected to blockchain');
    } finally {
      setIsConnecting(false);
    }
  }, []);

  const disconnect = useCallback(() => {
    setClient(null);
    setChainId(null);
    setWalletAddress(null);
    setError(null);
  }, []);

  return (
    <LineraContext.Provider
      value={{
        client,
        chainId,
        walletAddress,
        connect,
        disconnect,
        isConnected: !!client,
        isConnecting,
        error,
        appIds: APP_IDS,
        graphqlEndpoint: GRAPHQL_URL,
        faucetUrl: FAUCET_URL,
        isLocalNetwork,
      }}
    >
      {children}
    </LineraContext.Provider>
  );
}

export const useLinera = () => useContext(LineraContext);
