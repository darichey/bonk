import NavButton from "./components/NavButton";
import financeAppSvg from "./assets/finance-app.svg";
import MetadataSidebar from "./components/MetadataSidebar";
import DashboardSidebar from "./components/DashboardSidebar";
import { Link, Outlet } from "react-router-dom";

export default function App() {
  return (
    <div className="flex flex-row h-full w-full px-2">
      <div className="flex flex-col border-r-2 w-1/12 px-2">
        <img src={financeAppSvg} alt="finance-app logo" className="py-4" />
        <ul className="mb-5">
          <li>
            <Link to="/">Home</Link>
          </li>
          <li>
            <Link to="/log">Log</Link>
          </li>
          <li>
            <Link to="/query">Query</Link>
          </li>
          <li>
            <Link to="/import">Import</Link>
          </li>
        </ul>
        <div className="mb-5">
          <MetadataSidebar />
        </div>
        <div className="mb-5">
          <DashboardSidebar />
        </div>
      </div>
      <div className="flex flex-col flex-grow">
        <div className="flex flex-row border-b-2">
          <NavButton dir="back" className="border-r-2" />
          <NavButton dir="forward" className="border-r-2" />
        </div>
        <main className="h-full overflow-y-auto p-4">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
