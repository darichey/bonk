import { DataGrid, GridColDef } from "@mui/x-data-grid";
import { useDb } from "../db/context";
import { useQuery } from "../util/useQuery";

export default function Log() {
  const db = useDb();
  const result = useQuery(() => db.fetchAllTransactions());

  if (result.type === "loading") {
    return <div>Loading...</div>;
  }

  if (result.type === "error") {
    return (
      <div>
        <div>Error:</div>
        {JSON.stringify(result.error)}
      </div>
    );
  }

  const columns: GridColDef[] = [
    { field: "id" },
    { field: "date" },
    { field: "description" },
    { field: "account" },
    { field: "amount" },
  ];

  return (
    <div>
      <div>Log</div>
      <DataGrid rows={result.ok} columns={columns} />
    </div>
  );
}
