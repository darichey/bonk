"use client";

// TODO: figure out tree-shaking
import "chart.js/auto";

import React from "react";
import { ChartData, ChartOptions } from "chart.js";
import { Line } from "react-chartjs-2";
import { useQueryTransactionsForChart } from "./commands";

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

  const options: ChartOptions<"line"> = {
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

  const data: ChartData<"line", (number | string | null)[], unknown> = {
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

  return <Line options={options} data={data} />;
}
