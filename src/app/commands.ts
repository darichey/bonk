import { invoke } from "@tauri-apps/api/tauri";
import useSWR from "swr";
import { Transaction } from "./types";

export function useGetAllTransactions() {
  return useSWR<Transaction[]>("get_all_transactions", invoke);
}

export function useQueryTransactionsForChart(query: string) {
  return useSWR<Record<string, (number | string | null)[]>>(
    ["query_transactions_for_chart", query],
    ([cmd, arg]) => {
      return invoke(cmd, { query: arg });
    }
  );
}

export function useQueryTransactions(query: string) {
  return useSWR<[string[], (number | string | null)[][]]>(
    ["query_transactions", query],
    ([cmd, arg]) => {
      return invoke(cmd, { query: arg });
    }
  );
}
