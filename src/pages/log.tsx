import { fetchAllTransactions } from "../db/database";
import { Transaction } from "../db/model";
import { useEffect, useState } from "react";

export default function Log() {
  const [transactions, setTransactions] = useState<Transaction[]>([]);

  useEffect(() => {
    fetchAllTransactions().then(setTransactions);
  }, [setTransactions]);

  return (
    <div>
      <div>Log</div>
      <div>
        {transactions.map((transaction) => (
          <div>{transaction.id}</div>
        ))}
      </div>
    </div>
  );
}
