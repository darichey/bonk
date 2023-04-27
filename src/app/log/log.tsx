import { DataGrid, GridColDef } from "@mui/x-data-grid";
import { Database } from "../../db/database";

export default async function Log() {
  const db = await Database.withDummyData();
  const rows = await db.fetchAllTransactions();

  const columns: GridColDef[] = [
    { field: "id" },
    { field: "date" },
    { field: "description" },
    { field: "account" },
    { field: "amount" },
  ];

  return <DataGrid rows={rows} columns={columns} />;
}
