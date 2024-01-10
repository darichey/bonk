import { useParams } from "react-router-dom";
import Dashboard from "../components/Dashboard";

export default function DashboardPage() {
  const { name } = useParams();

  if (!name) {
    return <div>Error: no dashboard name</div>;
  }

  return <Dashboard name={name} />;
}
