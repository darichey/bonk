"use client";

// TODO: figure out tree-shaking
import "chart.js/auto";

import { ChartData } from "chart.js";
import { Line } from "react-chartjs-2";
import { useQueryTransactionsForChart } from "@/commands";
import { SqlValue } from "@/types";
import { getLineOrBarOptions } from "./common";

export default function QueryLineChart({
  title,
  xAxis,
  query,
}: {
  title: string;
  xAxis: string;
  query: string;
}) {
  const {
    data: chartData,
    error,
    isLoading,
  } = useQueryTransactionsForChart(query);

  if (error) {
    return <div>Encountered error: {error}</div>;
  }

  if (isLoading || !chartData) {
    return <div>Loading...</div>;
  }

  const data: ChartData<"line", SqlValue[], unknown> = {
    labels: chartData[xAxis],
    datasets: Object.keys(chartData)
      .filter((key) => key != xAxis)
      .map((key) => {
        return {
          label: key,
          data: chartData[key],
        };
      }),
  };

  return <Line options={getLineOrBarOptions(title)} data={data} />;
}
