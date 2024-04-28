"use client";

import { useGetDashboard, useLiveReload } from "@/commands";
import DashboardTable from "@/components/DashboardTable";
import DashboardText from "@/components/DashboardText";
import QueryBarChart from "@/components/chart/QueryBarChart";
import QueryLineChart from "@/components/chart/QueryLineChart";
import QueryStackedBarChart from "@/components/chart/QueryStackedBarChart";
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
            {component.type === "barChart" ? (
              <QueryBarChart
                title={component.title}
                xAxis={component.xAxis}
                query={component.query}
              />
            ) : component.type === "lineChart" ? (
              <QueryLineChart
                title={component.title}
                xAxis={component.xAxis}
                query={component.query}
              />
            ) : component.type === "stackedBarChart" ? (
              <QueryStackedBarChart
                title={component.title}
                xAxis={component.xAxis}
                stackKey={component.stackKey}
                query={component.query}
              />
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
