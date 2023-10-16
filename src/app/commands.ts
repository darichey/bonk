import { invoke } from "@tauri-apps/api/tauri";
import useSWR from "swr";
import { ChartData, SqlValue, TableData, Transaction } from "./types";

export function useGetAllTransactions() {
  return useSWR<Transaction[]>("get_all_transactions", invoke);
}

export function useQueryTransactionsForChart(query: string) {
  return useSWR<ChartData>(
    ["query_transactions_for_chart", query],
    ([cmd, arg]) => {
      return invoke(cmd, { query: arg });
    }
  );
}

export function useQueryTransactions(query: string) {
  return useSWR<TableData>(["query_transactions", query], ([cmd, arg]) => {
    return invoke(cmd, { query: arg });
  });
}

export function useGetMetadataNames() {
  return useSWR<string[]>("get_metadata_names", invoke);
}

export function useGetMetadata(name: string) {
  return useSWR<TableData>(["get_metadata", name], ([cmd, arg]) => {
    return invoke(cmd, { name: arg });
  });
}
