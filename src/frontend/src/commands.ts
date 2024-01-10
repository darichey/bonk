import useSWR, { SWRResponse } from "swr";
import { ChartData, Dashboard, TableData, Transaction } from "./types";

export function useGetAllTransactions(): SWRResponse<Transaction[]> {
  throw new Error("unimplemented"); // TODO
}

export function useQueryTransactionsForChart(
  query: string
): SWRResponse<ChartData> {
  throw new Error("unimplemented"); // TODO
}

export function useQueryTransactions(query: string): SWRResponse<TableData> {
  throw new Error("unimplemented"); // TODO
}

export function useGetDashboardNames(): SWRResponse<string[]> {
  throw new Error("unimplemented"); // TODO
}

export function useGetDashboard(name: string): SWRResponse<Dashboard> {
  throw new Error("unimplemented"); // TODO
}

export function useRenderQueryTemplate(
  template: string,
  variables: Record<string, string>
): SWRResponse<string> {
  throw new Error("unimplemented"); // TODO
}
