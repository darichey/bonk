// TODO: figure out tree-shaking
import "chart.js/auto";

import { ChartData, ChartOptions } from "chart.js";
import { Line } from "react-chartjs-2";
import { useQueryTransactionsForChart } from "../commands";
import { SqlValue } from "../types";
import { formatDollar } from "../util";

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
      tooltip: {
        callbacks: {
          label(context) {
            let label = context.dataset.label || "";

            if (label) {
              label += ": ";
            }

            label += formatDollar(context.parsed.y);

            return label;
          },
        },
      },
    },
    animation: false,
    scales: {
      y: {
        ticks: {
          callback(value) {
            if (typeof value === "string") {
              throw new Error("QueryLineChart y tick shouldn't be string.");
            }
            return formatDollar(value);
          },
        },
      },
    },
  };

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

  return <Line options={options} data={data} />;
}
