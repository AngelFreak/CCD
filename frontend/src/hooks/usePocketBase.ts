import { useState, useEffect } from 'react';
import pb from '../lib/pocketbase';

export function usePocketBase() {
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    checkConnection();
  }, []);

  async function checkConnection() {
    try {
      await pb.health.check();
      setIsConnected(true);
    } catch (err) {
      setIsConnected(false);
    }
  }

  return {
    pb,
    isConnected,
    checkConnection,
  };
}
