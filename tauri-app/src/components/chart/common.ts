import { ChartOptions } from "chart.js";
import { formatDollar } from "../../util";

export function getLineOrBarOptions(
  title: string
): ChartOptions<"line" | "bar"> {
  return {
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
}
