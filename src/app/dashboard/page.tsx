import { Metadata } from "next";
import QueryChart from "../components/QueryChart";

export const metadata: Metadata = {
  title: "Dashboard",
};

export default function DashboardPage({
  searchParams,
}: {
  searchParams: { [key: string]: string | string[] | undefined };
}) {
  const name = searchParams["name"] as string; // TODO

  return (
    <div>
      <QueryChart title={name} />
    </div>
  );
}
