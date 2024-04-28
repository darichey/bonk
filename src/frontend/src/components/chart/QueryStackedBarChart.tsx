"use client";

import { Bar } from "react-chartjs-2";
import { useQueryTransactions, useQueryTransactionsForChart } from "@/commands";
import { ChartData } from "chart.js";
import { SqlValue } from "@/types";
import { getLineOrBarOptions } from "./common";

export default function QueryBarChart({
  title,
  xAxis,
  stackKey,
  query,
}: {
  title: string;
  xAxis: string;
  stackKey: string;
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

  console.log(chartData);

  const data: ChartData<"bar", SqlValue[], unknown> = {
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

  return <Bar options={getLineOrBarOptions(title)} data={data} />;

  //   return <div></div>;
}
