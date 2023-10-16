"use client";

import { Bar } from "react-chartjs-2";
import { useQueryTransactionsForChart } from "../commands";
import { ChartData, ChartOptions } from "chart.js";
import { SqlValue } from "../types";

export default function QueryBarChart({
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

  const options: ChartOptions<"bar"> = {
    responsive: true,
    plugins: {
      legend: {
        position: "top",
      },
      title: {
        display: true,
        text: title,
      },
    },
    animation: false,
  };

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

  return <Bar options={options} data={data} />;
}
