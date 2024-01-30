"use client";

import Link from "next/link";
import { useGetDashboardNames } from "@/commands";

export default function DashboardSidebar() {
  const { data: dashboardNames, isLoading, error } = useGetDashboardNames();

  return (
    <div>
      <div className="text-xs text-gray-500 border-b">Dashboards</div>
      {error ? (
        <div>Encountered error: {error}</div>
      ) : isLoading || !dashboardNames ? (
        <div>Loading...</div>
      ) : (
        <ul>
          {dashboardNames.map((name, i) => (
            <Link href={`/dashboard/${name}`} key={i}>
              {name}
            </Link>
          ))}
        </ul>
      )}
    </div>
  );
}
