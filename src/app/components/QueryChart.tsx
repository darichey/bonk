"use client";

import { useGetDashboard } from "../commands";
import QueryBarChart from "./QueryBarChart";
import QueryLineChart from "./QueryLineChart";

export default function QueryChart({ title }: { title: string }) {
  const { data: dashboard, isLoading, error } = useGetDashboard(title);

  return error ? (
    <div>Encountered error: {error}</div>
  ) : isLoading || !dashboard ? (
    <div>Loading...</div>
  ) : dashboard.chartType === "bar" ? (
    <QueryBarChart
      title={title}
      xAxis={dashboard.xAxis}
      query={dashboard.query}
    />
  ) : dashboard.chartType === "line" ? (
    <QueryLineChart
      title={title}
      xAxis={dashboard.xAxis}
      query={dashboard.query}
    />
  ) : (
    <div>Unknown chart type: '{dashboard.chartType}'</div>
  );
}
