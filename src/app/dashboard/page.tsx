import { Metadata } from "next";
import Dashboard from "../components/Dashboard";

export const metadata: Metadata = {
  title: "Dashboard",
};

export default function DashboardPage({
  searchParams,
}: {
  searchParams: { [key: string]: string | string[] | undefined };
}) {
  const name = searchParams["name"] as string; // TODO

  return <Dashboard name={name} />;
}
