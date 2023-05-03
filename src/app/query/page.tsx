"use client";

import { Button, TextField } from "@mui/material";
import { useState } from "react";
import { getDatabase } from "../../db/database";

export default function Query() {
  const [query, setQuery] = useState("");
  const [result, setResult] = useState("");

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
    <div>
      <div>Query page</div>
      <TextField
        multiline
        fullWidth
        minRows={5}
        inputProps={{
          style: {
            fontFamily: "monospace",
          },
        }}
        onChange={(event) => setQuery(event.target.value)}
      />
      <Button variant="contained" onClick={onSubmit}>
        Submit
      </Button>
      <div>Result</div>
      <div>{result}</div>
    </div>
  );
}
