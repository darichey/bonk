"use client";

import { useGetAllTransactions } from "@/commands";
import { Transaction } from "@/types";
import { useState } from "react";

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

function PaginationButtons({
  onPrevious,
  onNext,
}: {
  onPrevious: () => void;
  onNext: () => void;
}) {
  return (
    <div className="flex justify-between mt-4">
      <button onClick={onPrevious} className="btn">
        Previous
      </button>
      <button onClick={onNext} className="btn">
        Next
      </button>
    </div>
  );
}

export default function LogPage() {
  const today = new Date().toISOString().split("T")[0];
  const [beforeDate, setBeforeDate] = useState<string | null>(today);
  const [afterDate, setAfterDate] = useState<string | null>(null);
  let pageDate;
  if (beforeDate) {
    pageDate = { type: "before", beforeDate };
  } else if (afterDate) {
    pageDate = { type: "after", afterDate };
  }
  const limit = 100; // Limit to 100 items per page
  const {
    data: transactions,
    error,
    isLoading,
  } = useGetAllTransactions(pageDate, limit);

  const handleNextPage = () => {
    if (transactions && transactions.length > 0) {
      const lastTransactionDate = transactions[transactions.length - 1].date;
      setBeforeDate(lastTransactionDate);
      setAfterDate(null);
    }
  };

  const handlePreviousPage = () => {
    if (transactions && transactions.length > 0) {
      const firstTransactionDate = transactions[0].date;
      setAfterDate(firstTransactionDate);
      setBeforeDate(null);
    }
  };

  return isLoading || !transactions ? (
    <div>transactions here... loading...</div>
  ) : error ? (
    <div>Encountered error: {error}</div>
  ) : (
    <div className="p-6">
      <PaginationButtons
        onPrevious={handlePreviousPage}
        onNext={handleNextPage}
      />
      <div className="flex flex-col gap-4">
        {transactions.map((transaction, idx) => (
          <TransactionView transaction={transaction} key={idx} />
        ))}
      </div>
      <PaginationButtons
        onPrevious={handlePreviousPage}
        onNext={handleNextPage}
      />
    </div>
  );
}
