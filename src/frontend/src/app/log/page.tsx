"use client";

import { useGetAllTransactions } from "@/commands";
import { Transaction } from "@/types";

function TransactionView({ transaction }: { transaction: Transaction }) {
  return (
    <div className="border-b-2 border-gray-300 p-4">
      <div className="font-bold text-lg mb-2 flex justify-between">
        <span>{transaction.description}</span>
        <span>{transaction.date}</span>
      </div>
      <div className="pl-4 font-mono">
        {transaction.postings.map((posting, idx) => (
          <div key={idx} className="flex justify-between py-1">
            <span className="text-gray-700">{posting.account}</span>
            <span className="text-gray-900">{showAmount(posting.amount)}</span>
          </div>
        ))}
      </div>
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
    <div className="p-6">
      <div className="flex flex-col gap-4">
        {transactions.map((transaction, idx) => (
          <TransactionView transaction={transaction} key={idx} />
        ))}
      </div>
    </div>
  );
}
