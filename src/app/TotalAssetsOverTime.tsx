"use client";

import React from "react";
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  ChartData,
  ChartOptions,
} from "chart.js";
import { Line } from "react-chartjs-2";
import { faker } from "@faker-js/faker";
import { useGetAllTransactions } from "./commands";

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

export const options: ChartOptions<"line"> = {
  responsive: true,
  plugins: {
    legend: {
      position: "top",
    },
    title: {
      display: true,
      text: "Total Assets Over Time",
    },
  },
  animation: false,
};

// export const data = {
//   labels,
//   datasets: [
//     {
//       label: "Dataset 1",
//       data: labels.map(() => faker.number.int({ min: -1000, max: 1000 })),
//       borderColor: "rgb(255, 99, 132)",
//       backgroundColor: "rgba(255, 99, 132, 0.5)",
//     },
//     {
//       label: "Dataset 2",
//       data: labels.map(() => faker.number.int({ min: -1000, max: 1000 })),
//       borderColor: "rgb(53, 162, 235)",
//       backgroundColor: "rgba(53, 162, 235, 0.5)",
//     },
//   ],
// };

export default function TotalAssetsOverTime() {
  const { data: transactions, error, isLoading } = useGetAllTransactions();

  if (isLoading || !transactions) {
    return <div>Loading...</div>;
  }

  if (error) {
    return <div>Encountered error: {error}</div>;
  }

  const data: ChartData<"line", number[], unknown> = {
    labels: transactions.map((transaction) => transaction.date),
    datasets: [
      {
        label: "Assets",
        data: transactions
          .map((transaction) => transaction.amount.cents)
          .map(
            (
              (sum) => (value) =>
                (sum += value)
            )(0)
          ),
      },
    ],
  };

  return <Line options={options} data={data} />;
}
