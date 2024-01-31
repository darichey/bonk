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
  return useSWR(
    "useQueryTransactions",
    () => ({ column_names: [], data: [] } as TableData)
  ); // TODO
}

export function useGetDashboardNames(): SWRResponse<string[]> {
  return useSWR("useGetDashboardNames", () => [] as string[]); // TODO
}

export function useGetDashboard(name: string): SWRResponse<Dashboard> {
  return useSWR(
    "useGetDashboard",
    () => ({ name: "TODO", components: [] } as Dashboard)
  ); // TODO
}

export function useRenderQueryTemplate(
  template: string,
  variables: Record<string, string>
): SWRResponse<string> {
  return useSWR("useRenderQueryTemplate", () => "TODO"); // TODO
}
