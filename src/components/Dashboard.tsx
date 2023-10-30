import { useGetDashboard } from "../commands";
import QueryChart from "./QueryChart";

export default function Dashboard({ name }: { name: string }) {
  const { data: dashboard, isLoading, error } = useGetDashboard(name);

  return error ? (
    <div>Encountered error: {error}</div>
  ) : isLoading || !dashboard ? (
    <div>Loading...</div>
  ) : (
    <div>
      <h1>{dashboard.name}</h1>
      {dashboard.charts.map((chart, i) => (
        <QueryChart key={i} chart={chart} />
      ))}
    </div>
  );
}
