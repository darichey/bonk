import { default as TauriDatabase } from "tauri-plugin-sql-api";
import { Transaction } from "./model";

export class Database {
  static async withDummyData(): Promise<Database> {
    const db = await TauriDatabase.load("sqlite:test.db");
    return new Database(db);
  }

  _db: TauriDatabase;

  constructor(_db: TauriDatabase) {
    this._db = _db;
  }

  fetchAllTransactions(): Promise<Transaction[]> {
    return this._db.select<Transaction[]>(`SELECT * FROM "transaction"`);
  }

  close(): Promise<boolean> {
    return this._db.close();
  }
}
