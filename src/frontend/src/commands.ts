"use client";

import useSWR, { SWRResponse, useSWRConfig } from "swr";
import { ChartData, Dashboard, TableData, Transaction } from "./types";
import { useEffect } from "react";

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
  return useSWR(["/queryTransactionsForChart", query], async ([_, query]) => {
    const res = await fetch(`http://localhost:8080/queryTransactionsForChart`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ query }),
    });
    const json = await res.json();
    return json as ChartData;
  });
}

export function useQueryTransactions(query: string): SWRResponse<TableData> {
  return useSWR(["/queryTransactions", query], async ([_, query]) => {
    if (!query) {
      return {
        columnNames: [],
        data: [],
      };
    }

    const res = await fetch(`http://localhost:8080/queryTransactions`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ query }),
    });

    const json = await res.json();

    if (res.ok) {
      return json as TableData;
    } else {
      throw new Error(json);
    }
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
  return useSWR(
    ["/renderQueryTemplate", template, variables],
    async ([_, template, variables]) => {
      const res = await fetch(`http://localhost:8080/renderQueryTemplate`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ template, variables }),
      });
      const json = await res.json();
      return json as string;
    }
  );
}

// FIXME: causes two reloads
export function useLiveReload() {
  const { mutate } = useSWRConfig();

  useEffect(() => {
    const eventSource = new EventSource(`http://localhost:8080/liveReload`);

    eventSource.addEventListener("message", (_event) => {
      mutate(() => true, undefined, { revalidate: true });
    });

    return () => {
      eventSource.close();
    };
  }, []);
}
