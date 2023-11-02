import { useGetDashboard } from "../commands";
import DashboardText from "./DashboardText";
import QueryChart from "./chart/QueryChart";

export default function Dashboard({ name }: { name: string }) {
  const { data: dashboard, isLoading, error } = useGetDashboard(name);

  return error ? (
    <div>Encountered error: {error}</div>
  ) : isLoading || !dashboard ? (
    <div>Loading...</div>
  ) : (
    <div>
      <h1>{dashboard.name}</h1>
      {dashboard.components.map((component, i) => {
        switch (component.type) {
          case "chart":
            return <QueryChart key={i} chart={component} />;
          case "text":
            return <DashboardText key={i} text={component} />;
        }
      })}
    </div>
  );
}
