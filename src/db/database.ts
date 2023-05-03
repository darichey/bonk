import { readTextFile } from "@tauri-apps/api/fs";
import { resolveResource } from "@tauri-apps/api/path";
import { cache } from "react";
import { default as TauriDatabase } from "tauri-plugin-sql-api";
import { Transaction } from "./model";

export class Database {
  _db: TauriDatabase;

  constructor(_db: TauriDatabase) {
    this._db = _db;
  }

  allTransactions(): Promise<Transaction[]> {
    return this.query(`SELECT * FROM "transaction"`);
  }

  query<T>(query: string, bindValues?: unknown[]): Promise<T> {
    return this._db.select<T>(query, bindValues);
  }

  close(): Promise<boolean> {
    return this._db.close();
  }
}

export const getDatabase = cache(async () => {
  const db = await TauriDatabase.load("sqlite:test.db");
  await db.execute(`drop table if exists account`);
  await db.execute(`drop table if exists "transaction"`);
  const schemaRes = await resolveResource("../sql/schema.sql");
  const schemaStr = await readTextFile(schemaRes);
  const r = await db.execute(schemaStr);
  console.log(r);
  const dummyRes = await resolveResource("../sql/dummy.sql");
  const dummyStr = await readTextFile(dummyRes);
  const r1 = await db.execute(dummyStr);
  console.log(r1);
  return new Database(db);
});
