import { useEffect, useState } from "react";

export type QueryResult<T> =
  | {
      type: "ok";
      ok: T;
    }
  | {
      type: "error";
      error: unknown;
    }
  | {
      type: "loading";
    };

export function useQuery<T>(f: () => Promise<T>) {
  const [result, setResult] = useState<QueryResult<T>>({ type: "loading" });

  useEffect(() => {
    (async () => {
      try {
        const ok = await f();
        setResult({ type: "ok", ok });
      } catch (error) {
        setResult({ type: "error", error });
      }
    })();
  }, [setResult]);

  return result;
}
