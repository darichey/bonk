import { DataGrid, GridColDef } from "@mui/x-data-grid";
import { useDb } from "../db/context";
import { useQuery } from "../util/useQuery";

export default function Log() {
  const db = useDb();
  const {
    isLoading,
    result: transactions,
    error,
  } = useQuery(() => db.fetchAllTransactions());

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (error) {
    return (
      <div>
        <div>Error:</div>
        {JSON.stringify(error)}
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
      <DataGrid rows={transactions} columns={columns} />
    </div>
  );
}
