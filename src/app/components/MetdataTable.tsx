"use client";

import { useGetMetadata } from "../commands";

export default function MetadataTable({ name }: { name: string }) {
  const { data, isLoading, error } = useGetMetadata(name);

  return error ? (
    <div>Encountered error: {error}</div>
  ) : isLoading || !data ? (
    <div>Loading...</div>
  ) : (
    // TODO: dedupe code for tables
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
  );
}
