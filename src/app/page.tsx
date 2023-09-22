"use client";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface Transaction {
  date: string;
  description: string;
  amount: {
    cents: number;
  };
  account: string;
}

export default function Home() {
  const [isLoading, setIsLoading] = useState(true);
  const [transactions, setTransactions] = useState<Transaction[]>([]);

  useEffect(() => {
    invoke<Transaction[]>("get_all_transactions").then((transactions) => {
      setTransactions(transactions);
      setIsLoading(false);
    });
  }, [setIsLoading, setTransactions]);

  return (
    <div>
      <h1 className="font-bold">Your Transaction Log</h1>
      {isLoading ? (
        <div>transactions here... loading...</div>
      ) : (
        <table>
          <tr>
            <th>Date</th>
            <th>Description</th>
            <th>Account</th>
            <th>Amount</th>
          </tr>
          {transactions.map((transaction, i) => (
            <tr key={i}>
              <td>{transaction.date}</td>
              <td>{transaction.description}</td>
              <td>{transaction.account}</td>
              <td>{transaction.amount.cents}</td>
            </tr>
          ))}
        </table>
      )}
    </div>
  );
}
