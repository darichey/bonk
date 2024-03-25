"use client";

import { useGetAllTransactions } from "@/commands";
import { Transaction } from "@/types";

function TransactionView({ transaction }: { transaction: Transaction }) {
  return (
    <div className="border-b-black border-b-2">
      <div>
        {transaction.date} {transaction.description}
      </div>
      {transaction.postings.map((posting, idx) => (
        <div key={idx}>
          {posting.account} {showAmount(posting.amount)}
        </div>
      ))}
    </div>
  );
}

function showAmount(amount: number): string {
  let dollars = Math.round(amount / 100);
  let cents = Math.abs(amount) % 100;
  return `${dollars}.${cents.toString().padStart(2, "0")}`;
}

export default function LogPage() {
  const { data: transactions, error, isLoading } = useGetAllTransactions(); // TODO: paginate

  return isLoading || !transactions ? (
    <div>transactions here... loading...</div>
  ) : error ? (
    <div>Encountered error: {error}</div>
  ) : (
    <div>
      <div>
        <b>{transactions.length} transactions</b>
      </div>
      <div className="flex flex-col gap-2">
        {transactions
          .toSorted((a, b) => a.date.localeCompare(b.date)) // TODO: sorting should be done on the server
          .map((transaction, idx) => (
            <TransactionView transaction={transaction} key={idx} />
          ))}
        div
      </div>
    </div>
  );
}
