import { Transaction } from "./model";
import Database from "tauri-plugin-sql-api";

export async function fetchAllTransactions(): Promise<Transaction[]> {
  const db = await Database.load("sqlite:test.db");
  const res = await db.select<Transaction[]>("SELECT * FROM transaction");
  await db.close();
  return res;
}
