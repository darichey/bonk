"use client";

import { useGetAllTransactions } from "./commands";

export default function Home() {
  const { data: transactions, error, isLoading } = useGetAllTransactions();

  return (
    <div>
      <h1 className="font-bold">Your Transaction Log</h1>
      {isLoading ? (
        <div>transactions here... loading...</div>
      ) : error ? (
        <div>Encountered error: {error}</div>
      ) : (
        <table>
          <tr>
            <th>Date</th>
            <th>Description</th>
            <th>Account</th>
            <th>Amount</th>
          </tr>
          {transactions?.map((transaction, i) => (
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
