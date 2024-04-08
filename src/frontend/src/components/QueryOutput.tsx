import { useQueryTransactions } from "@/commands";

export function QueryOutput({ query }: { query: string }) {
  const { data, error, isLoading } = useQueryTransactions(query);

  return error ? (
    <div>Encountered error: {error.toString()}</div>
  ) : isLoading || !data ? (
    <div>Loading...</div>
  ) : (
    <table>
      <thead>
        <tr>
          {data.columnNames.map((column, i) => (
            <th key={i}>{column}</th>
          ))}
        </tr>
      </thead>
      <tbody>
        {data.data.map((row, i) => (
          <tr key={i}>
            {row.map((col, j) => (
              <td key={j}>{col}</td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  );
}
