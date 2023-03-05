import Database from "tauri-plugin-sql-api";
import { Transaction } from "./model";

export async function fetchAllTransactions(): Promise<Transaction[]> {
  const db = await Database.load("sqlite:test.db");
  const res = await db.select<Transaction[]>("SELECT * FROM transaction");
  await db.close();
  return res;
}
