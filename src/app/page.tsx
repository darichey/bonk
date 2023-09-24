"use client";

import QueryBarChart from "./QueryBarChart";
import QueryLineChart from "./QueryLineChart";

export default function Home() {
  return (
    <main>
      <QueryLineChart
        title="Daily Total Assets"
        dataLabel="Total Assets"
        query={`
          SELECT date as x, SUM(SUM(amount)) OVER (ORDER BY date) as y
          FROM transactions
          GROUP BY x
          ORDER BY x
          `}
      />
      <QueryLineChart
        title="Monthly Total Assets"
        dataLabel="Total Assets"
        query={`
          SELECT STRFTIME('%Y-%m', DATE(date, 'start of month')) as x, SUM(SUM(amount)) OVER (ORDER BY date) as y
          FROM transactions
          GROUP BY x
          ORDER BY x
          `}
      />
      {/* TODO: this query is misleading because we don't differentiate account transfers (e.g., transferring from on account to another is counted as an expenditure) */}
      <QueryBarChart
        title="Expenditures per month"
        dataLabel="Total spent"
        query={`
          SELECT STRFTIME('%Y-%m', DATE(date, 'start of month')) as x, SUM(amount) as y
          FROM transactions
          WHERE amount < 0
          GROUP BY x
          ORDER BY x;
          `}
      />
      <QueryBarChart
        title="Income per month"
        dataLabel="Total gained"
        query={`
          SELECT STRFTIME('%Y-%m', DATE(date, 'start of month')) as x, SUM(amount) as y
          FROM transactions
          WHERE amount > 0
          GROUP BY x
          ORDER BY x;
          `}
      />
      <QueryBarChart
        title="Savings per month"
        dataLabel="Total saved"
        query={`
          SELECT STRFTIME('%Y-%m', DATE(date, 'start of month')) as x, SUM(amount) as y
          FROM transactions
          GROUP BY x
          ORDER BY x;
          `}
      />
    </main>
  );
}
