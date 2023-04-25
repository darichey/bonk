import { useDb } from "../db/context";
import { Transaction } from "../db/model";
import { useQuery } from "../util/useQuery";
import { useContext, useEffect, useState } from "react";

export default function Log() {
  const db = useDb();
  const {
    isLoading,
    result: transactions,
    error,
  } = useQuery(() => db.fetchAllTransactions());

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (error) {
    return (
      <div>
        <div>Error:</div>
        {JSON.stringify(error)}
      </div>
    );
  }

  return (
    <div>
      <div>Log</div>
      <div>
        {transactions.map((transaction) => (
          <div key={transaction.id}>{transaction.id}</div>
        ))}
      </div>
    </div>
  );
}
