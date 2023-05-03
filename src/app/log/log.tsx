import { DataGrid, GridColDef } from "@mui/x-data-grid";
import { getDatabase } from "../../db/database";

export default async function Log() {
  const db = await getDatabase();
  const rows = await db.allTransactions();

  const columns: GridColDef[] = [
    { field: "id" },
    { field: "date" },
    { field: "description" },
    { field: "account" },
    { field: "amount" },
  ];

  return <DataGrid rows={rows} columns={columns} />;
}
