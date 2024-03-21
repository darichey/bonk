"use client";

import useSWR, { SWRResponse } from "swr";
import { ChartData, Dashboard, TableData, Transaction } from "./types";

export function useGetAllTransactions(): SWRResponse<Transaction[]> {
  return useSWR("/transactions", async () => {
    const res = await fetch(`http://localhost:8080/transactions`);
    const json = await res.json();
    return json as Transaction[];
  });
}

export function useQueryTransactionsForChart(
  query: string
): SWRResponse<ChartData> {
  return useSWR("useQueryTransactionsForChart", () => ({} as ChartData)); // TODO
}

export function useQueryTransactions(query: string): SWRResponse<TableData> {
  return useSWR(["/queryTransactions", query], async ([_, query]) => {
    const res = await fetch(`http://localhost:8080/queryTransactions`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ query }),
    });
    const json = await res.json();
    return json as TableData;
  });
}

export function useGetDashboardNames(): SWRResponse<string[]> {
  return useSWR("/dashboardNames", async (_) => {
    const res = await fetch(`http://localhost:8080/dashboardNames`);
    const json = await res.json();
    return json as string[];
  });
}

export function useGetDashboard(name: string): SWRResponse<Dashboard> {
  return useSWR(["/dashboard", name], async ([_, name]) => {
    const res = await fetch(`http://localhost:8080/dashboard`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ name }),
    });
    const json = await res.json();
    return json as Dashboard;
  });
}

export function useRenderQueryTemplate(
  template: string,
  variables: Record<string, string>
): SWRResponse<string> {
  return useSWR("useRenderQueryTemplate", () => "TODO"); // TODO
}
