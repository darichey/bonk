import { invoke } from "@tauri-apps/api/tauri";
import useSWR from "swr";
import { ChartData, Dashboard, TableData, Transaction } from "./types";

// TODO: annotate function return types

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

export function useGetDashboardNames() {
  return useSWR<string[]>("get_dashboard_names", invoke);
}

export function useGetDashboard(name: string) {
  return useSWR<Dashboard>(["get_dashboard", name], ([cmd, arg]) => {
    return invoke(cmd, { name: arg });
  });
}

export function useRenderQueryTemplate(
  template: string,
  variables: Record<string, string>
) {
  return useSWR<string>(
    ["render_query_template", template, variables],
    ([cmd, arg1, arg2]) => {
      return invoke(cmd, { template: arg1, variables: arg2 });
    }
  );
}

export function useCreateLinkToken() {
  return useSWR<string>("create_link_token", invoke);
}

export function useExchangePublicToken(publicToken: string | null) {
  return useSWR<string>(
    publicToken ? ["exchange_public_token", publicToken] : null,
    ([cmd, arg]) => {
      return invoke(cmd, { publicToken: arg });
    }
  );
}

// TODO doFetch is stupid
export function usePlaidGetTransactions(doFetch: boolean) {
  return useSWR<string>(doFetch ? "plaid_get_transactions" : null, invoke);
}
