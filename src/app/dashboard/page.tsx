import { Metadata } from "next";
import QueryBarChart from "../components/QueryBarChart";
import QueryLineChart from "../components/QueryLineChart";

export const metadata: Metadata = {
  title: "Dashboard",
};

export default function Dashboard() {
  return (
    <>
      <QueryLineChart
        title="Daily Total Assets"
        xAxis="date"
        query={`
              SELECT
                date,
                SUM(SUM(amount)) OVER (ORDER BY date) as total
              FROM transactions
              GROUP BY date
              ORDER BY date
              `}
      />
      <QueryLineChart
        title="Monthly Total Assets"
        xAxis="month"
        query={`
              SELECT
                STRFTIME('%Y-%m', DATE(date, 'start of month')) as month,
                SUM(SUM(amount)) OVER (ORDER BY date) as total
              FROM transactions
              GROUP BY month
              ORDER BY month
              `}
      />
      {/* TODO: this query is misleading because we don't differentiate account transfers (e.g., transferring from on account to another is counted as an expenditure) */}
      <QueryBarChart
        title="Expenditures per month"
        xAxis="month"
        query={`
              SELECT
                STRFTIME('%Y-%m', DATE(date, 'start of month')) as month,
                SUM(amount) as total
              FROM transactions
              WHERE amount < 0
              GROUP BY month
              ORDER BY month
              `}
      />
      <QueryBarChart
        title="Income per month"
        xAxis="month"
        query={`
              SELECT
                STRFTIME('%Y-%m', DATE(date, 'start of month')) as month,
                SUM(amount) as total
              FROM transactions
              WHERE amount > 0
              GROUP BY month
              ORDER BY month
              `}
      />
      <QueryBarChart
        title="Savings per month"
        xAxis="month"
        query={`
              SELECT
                STRFTIME('%Y-%m', DATE(date, 'start of month')) as month,
                SUM(amount) as total
              FROM transactions
              GROUP BY month
              ORDER BY month
              `}
      />
    </>
  );
}
