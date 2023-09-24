"use client";

import { Bar } from "react-chartjs-2";
import { useQueryTransactionsForChart } from "./commands";
import { ChartData, ChartOptions } from "chart.js";
import { getFooBar } from "./QueryLineChart";

export default function QueryBarChart({
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

  const [foo, bar] = getFooBar(chartData);

  const data: ChartData<"bar", number[], unknown> = {
    labels: foo,
    datasets: [
      {
        label: dataLabel,
        data: bar,
      },
    ],
  };

  return <Bar options={options} data={data} />;
}
