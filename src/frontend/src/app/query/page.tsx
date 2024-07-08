"use client";

import { useGetQuery, useGetQueryNames } from "@/commands";
import { QueryOutput } from "@/components/QueryOutput";
import { Editor } from "@monaco-editor/react";
import { useEffect, useState } from "react";
import { useDebounce } from "usehooks-ts";

export default function QueryPage() {
  const [selectedQuery, setSelectedQuery] = useState<string>("");

  const {
    data: queryNames,
    error: queryNamesError,
    isLoading: queryNamesIsLoading,
  } = useGetQueryNames();

  const { data: fetchedQuery } = useGetQuery(selectedQuery);

  const [query, setQuery] = useState('select * from "transaction"\nlimit 10');

  // TODO: is this awful?
  useEffect(() => {
    setQuery(fetchedQuery?.query ?? 'select * from "transaction"\nlimit 10');
  }, [setQuery, fetchedQuery]);

  const debouncedQuery = useDebounce(query, 500);

  return (
    <>
      {queryNamesError ? (
        <div>Encountered error: {queryNamesError.toString()}</div>
      ) : queryNamesIsLoading || !queryNames ? (
        <div>Loading...</div>
      ) : (
        <select
          value={selectedQuery}
          onChange={(e) => setSelectedQuery(e.target.value)}
        >
          <option value=""></option>
          {queryNames.map((name) => (
            <option key={name} value={name}>
              {name}
            </option>
          ))}
        </select>
      )}
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
        <QueryOutput query={debouncedQuery} />
      </div>
    </>
  );
}
