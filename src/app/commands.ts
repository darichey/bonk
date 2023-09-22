import { invoke } from "@tauri-apps/api/tauri";
import useSWR from "swr";
import { Transaction } from "./types";

export function useGetAllTransactions() {
  return useSWR<Transaction[]>("get_all_transactions", invoke);
}
