"use client";

import { useGetAllTransactions } from "@/commands";
import { Transaction } from "@/types";

function TransactionView({ transaction }: { transaction: Transaction }) {
  return (
    <div className="border-b-black border-b-2">
      <div>
        {transaction.date} {transaction.description}
      </div>
      {transaction.postings.map((posting) => (
        <div>
          {posting.account} {posting.amount}
        </div>
      ))}
    </div>
  );
}

export default function LogPage() {
  const { data: transactions, error, isLoading } = useGetAllTransactions();

  return isLoading || !transactions ? (
    <div>transactions here... loading...</div>
  ) : error ? (
    <div>Encountered error: {error}</div>
  ) : (
    <div className="flex flex-col gap-2">
      {transactions.map((transaction) => (
        <TransactionView transaction={transaction} />
      ))}
    </div>
  );
}
