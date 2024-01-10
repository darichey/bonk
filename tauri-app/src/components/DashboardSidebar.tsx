import { Link } from "react-router-dom";
import { useGetDashboardNames } from "../commands";

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
            <Link to={`/dashboard/${name}`} key={i}>
              {name}
            </Link>
          ))}
        </ul>
      )}
    </div>
  );
}
