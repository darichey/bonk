"use client";

import { useGetDashboard, useLiveReload } from "@/commands";
import DashboardTable from "@/components/DashboardTable";
import DashboardText from "@/components/DashboardText";
import QueryChart from "@/components/chart/QueryChart";
import { useSearchParams } from "next/navigation";

export default function DashboardPage() {
  useLiveReload();
  const searchParams = useSearchParams();
  const name = searchParams.get("name") ?? "";
  const { data: dashboard, isLoading, error } = useGetDashboard(name);

  return error ? (
    <div>Encountered error: {error}</div>
  ) : isLoading || !dashboard ? (
    <div>Loading...</div>
  ) : (
    <div>
      <h1 className="font-extrabold text-2xl border-b p-1 mb-4">
        {dashboard.name}
      </h1>
      <div className="grid grid-cols-6 gap-4">
        {dashboard.components.map((component, i) => (
          <div
            style={{
              gridColumn: component.gridColumn,
              gridRow: component.gridRow,
            }}
            key={i}
          >
            {component.type === "chart" ? (
              <QueryChart chart={component} />
            ) : component.type === "text" ? (
              <DashboardText text={component} />
            ) : component.type === "table" ? (
              <DashboardTable table={component}></DashboardTable>
            ) : (
              <div>Unknown component type</div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
