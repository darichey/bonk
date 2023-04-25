import { useEffect, useState } from "react";

export function useQuery<T>(f: () => Promise<T>) {
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [result, setResult] = useState<T>(undefined);
  const [error, setError] = useState<any>(undefined);

  useEffect(() => {
    async function g() {
      try {
        const p = f();
        setIsLoading(true);
        const r = await p;
        setIsLoading(false);
        setResult(r);
      } catch (e) {
        setIsLoading(false);
        setError(e);
      }
    }
    g();
  }, [setIsLoading, setResult, setError]);

  return { isLoading, result, error };
}
