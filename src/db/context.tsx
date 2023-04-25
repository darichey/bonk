import { Database } from "./database";
import { createContext, useContext, useEffect, useRef, useState } from "react";

const DatabaseContext = createContext<Database>(null);

export function DatabaseProvider({ children }) {
  const dbRef = useRef<Database>();

  useEffect(() => {
    (async () => {
      dbRef.current = await Database.withDummyData();
    })();

    return () => {
      console.log("closing");
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
