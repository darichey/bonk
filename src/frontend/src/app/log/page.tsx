import { useGetAllTransactions } from "@/commands";

export default function LogPage() {
  const { data: transactions, error, isLoading } = useGetAllTransactions();

  return (
    <div>
      {isLoading ? (
        <div>transactions here... loading...</div>
      ) : error ? (
        <div>Encountered error: {error}</div>
      ) : (
        <table>
          <thead>
            <tr>
              <th>Date</th>
              <th>Description</th>
              <th>Account</th>
              <th>Amount</th>
            </tr>
          </thead>
          <tbody>
            {transactions?.map((transaction, i) => (
              <tr key={i}>
                <td>{transaction.date}</td>
                <td>{transaction.description}</td>
                <td>{transaction.account}</td>
                <td>{transaction.amount.cents}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}
