"use client";

import useSWR, { Key, SWRResponse, useSWRConfig } from "swr";
import {
  ChartData,
  ChatResponse,
  Dashboard,
  Query,
  TableData,
  Transaction,
} from "./types";
import { useEffect } from "react";
import useSWRMutation, { SWRMutationResponse } from "swr/mutation";

const API_HOST = process.env.NEXT_PUBLIC_API_HOST || "localhost";
const API_PORT = process.env.NEXT_PUBLIC_API_PORT || "8080";
const BASE_URL = `http://${API_HOST}:${API_PORT}`;

export function useGetAllTransactions(
  pageDate:
    | { type: "before"; beforeDate: string }
    | { type: "after"; afterDate: string },
  limit: number
): SWRResponse<Transaction[]> {
  const query = new URLSearchParams({
    limit: limit.toString(),
  });
  if (pageDate.type === "before") {
    query.set("before_date", pageDate.beforeDate);
  } else if (pageDate.type === "after") {
    query.set("after_date", pageDate.afterDate);
  }

  return useSWR(`/transactions?${query.toString()}`, async () => {
    const res = await fetch(`${BASE_URL}/transactions?${query.toString()}`);
    const json = await res.json();
    return json as Transaction[];
  });
}

export function useQueryTransactionsForChart(
  query: string
): SWRResponse<ChartData> {
  return useSWR(["/queryTransactionsForChart", query], async ([_, query]) => {
    const res = await fetch(`${BASE_URL}/queryTransactionsForChart`, {
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
  return useSWR(
    query ? ["/queryTransactions", query] : null,
    async ([_, query]) => {
      const res = await fetch(`${BASE_URL}/queryTransactions`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ query }),
      });

      if (res.ok) {
        const json = await res.json();
        return json as TableData;
      } else {
        const body = await res.text();
        throw new Error(body);
      }
    }
  );
}

export function useGetDashboardNames(): SWRResponse<string[]> {
  return useSWR("/dashboardNames", async (_) => {
    const res = await fetch(`${BASE_URL}/dashboardNames`);
    const json = await res.json();
    return json as string[];
  });
}

export function useGetDashboard(name: string): SWRResponse<Dashboard> {
  return useSWR(["/dashboard", name], async ([_, name]) => {
    const res = await fetch(`${BASE_URL}/dashboard`, {
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
      const res = await fetch(`${BASE_URL}/renderQueryTemplate`, {
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

export function useLiveReload() {
  const { mutate } = useSWRConfig();

  useEffect(() => {
    const eventSource = new EventSource(`${BASE_URL}/liveReload`);

    eventSource.addEventListener("message", (_event) => {
      mutate(() => true, undefined, { revalidate: true });
    });

    return () => {
      eventSource.close();
    };
  }, [mutate]);
}

export function useGetQueryNames(): SWRResponse<string[]> {
  return useSWR("/queryNames", async (_) => {
    const res = await fetch(`${BASE_URL}/queryNames`);
    const json = await res.json();
    return json as string[];
  });
}

export function useGetQuery(name: string): SWRResponse<Query> {
  return useSWR(name ? ["/query", name] : null, async ([_, name]) => {
    const res = await fetch(`${BASE_URL}/query`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ name }),
    });
    const json = await res.json();
    return json as Query;
  });
}

export function useGetChatResponse(): SWRMutationResponse<
  string,
  any,
  Key,
  { prompt: string }
> {
  return useSWRMutation(
    "/chat",
    async (_, { arg: { prompt } }: { arg: { prompt: string } }) => {
      const res = await fetch(`${BASE_URL}/chat`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ prompt }),
      });
      const json = await res.json();
      return (json as ChatResponse).response;
    }
  );
}
