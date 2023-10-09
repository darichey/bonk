"use client";

import { Editor } from "@monaco-editor/react";
import { useState } from "react";
import { useQueryTransactions } from "../commands";
import { useDebounce } from "usehooks-ts";

export default function Query() {
  const [query, setQuery] = useState("select * from transactions\nlimit 10");
  const debouncedQuery = useDebounce(query, 500);
  const { data, error, isLoading } = useQueryTransactions(debouncedQuery);

  return (
    <>
      <div className="h-1/4 border-b-2 p-2">
        <Editor
          options={{
            minimap: {
              enabled: false,
            },
            scrollBeyondLastLine: false,
          }}
          language="sql"
          value={query}
          onChange={(value) => {
            setQuery(value ?? "");
          }}
        />
      </div>
      <div className="p-2">
        {error ? (
          <div>Encountered error: {error}</div>
        ) : isLoading || !data ? (
          <div>Loading...</div>
        ) : (
          <table>
            <thead>
              <tr>
                {data[0].map((column, i) => (
                  <th key={i}>{column}</th>
                ))}
              </tr>
            </thead>
            <tbody>
              {data[1].map((row, i) => (
                <tr key={i}>
                  {row.map((col, j) => (
                    <td key={j}>{col}</td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </>
  );
}