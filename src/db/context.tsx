import { createContext, useContext, useEffect, useRef } from "react";
import { Database } from "./database";

const DatabaseContext = createContext<Database>(null);

export function DatabaseProvider({ children }) {
  const dbRef = useRef<Database>();

  useEffect(() => {
    (async () => {
      dbRef.current = await Database.withDummyData();
    })();

    return () => {
      dbRef.current?.close();
    };
  }, [dbRef]);

  return (
    <DatabaseContext.Provider value={dbRef.current}>
      {children}
    </DatabaseContext.Provider>
  );
}

export function useDb() {
  return useContext(DatabaseContext);
}
