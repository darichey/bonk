"use client";

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
    </main>
  );
}
