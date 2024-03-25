import { useQueryTransactions } from "@/commands";
import { Table } from "@/types";

export default function DashboardTable({
  table: { title, query },
}: {
  table: Table;
}) {
  const { data, error, isLoading } = useQueryTransactions(query);

  return error ? (
    <div>Encountered error: {error.toString()}</div>
  ) : isLoading || !data ? (
    <div>Loading...</div>
  ) : (
    <div className="flex flex-col items-center">
      <div>
        <b className="underline">{title}</b>
      </div>
      <table className="w-full">
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
    </div>
  );
}
