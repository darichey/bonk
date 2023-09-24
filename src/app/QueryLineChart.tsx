"use client";

// TODO: figure out tree-shaking
import "chart.js/auto";

import React from "react";
import { ChartData, ChartOptions } from "chart.js";
import { Line } from "react-chartjs-2";
import { useQueryTransactionsForChart } from "./commands";

export default function QueryLineChart({
  title,
  dataLabel,
  query,
}: {
  title: string;
  dataLabel: string;
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

  const [foo, bar] = getFooBar(chartData);

  const data: ChartData<"line", number[], unknown> = {
    labels: foo,
    datasets: [
      {
        label: dataLabel,
        data: bar,
      },
    ],
  };

  return <Line options={options} data={data} />;
}

function getFooBar(chartData: [string, number][]): [string[], number[]] {
  const foo = new Array<string>();
  const bar = new Array<number>();
  for (const [f, b] of chartData) {
    foo.push(f);
    bar.push(b);
  }
  return [foo, bar];
}
