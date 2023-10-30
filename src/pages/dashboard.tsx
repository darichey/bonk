import { useSearchParams } from "react-router-dom";
import Dashboard from "../components/Dashboard";

export default function DashboardPage() {
  const [searchParams, _] = useSearchParams();
  const name = searchParams.get("name");

  if (!name) {
    return <div>Error: no dashboard name</div>;
  }

  return <Dashboard name={name} />;
}
