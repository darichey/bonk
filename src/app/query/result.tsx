"use server";

import { Button } from "@mui/material";
import { getDatabase } from "../../db/database";

export default async function Result({
  result,
  setResult,
  query,
}: {
  result: string;
  setResult: (result: string) => void;
  query: string;
}) {
  async function onSubmit() {
    try {
      const db = await getDatabase();
      const result = await db.query(query);
      setResult(JSON.stringify(result));
    } catch (e) {
      setResult(JSON.stringify(e));
    }
  }

  return (
    <>
      <Button variant="contained" onClick={onSubmit}>
        Submit
      </Button>
      <div>Result</div>
      <div>{result}</div>
    </>
  );
}
