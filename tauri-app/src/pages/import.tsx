import { useCallback, useEffect, useState } from "react";
import { PlaidLinkOnSuccessMetadata, usePlaidLink } from "react-plaid-link";
import useSWR from "swr";
import {
  useCreateLinkToken,
  useExchangePublicToken,
  usePlaidGetTransactions,
} from "../commands";

export default function ImportPage() {
  const [showImport, setShowImport] = useState(false);

  return (
    <div>
      <h1 className="text-xl">Import</h1>
      <button onClick={() => setShowImport(true)}>Import From Plaid</button>

      {showImport ? <CreateToken /> : null}
    </div>
  );
}

function CreateToken() {
  const {
    data: linkToken,
    isLoading: isLoadingLinkToken,
    error: errorLinkToken,
  } = useCreateLinkToken();

  const [publicToken, setPublicToken] = useState<string | null>(null);
  const isLinked = publicToken != null;

  const onSuccess = useCallback(
    (publicToken: string) => {
      setPublicToken(publicToken);
    },
    [setPublicToken]
  );

  const { open, ready } = usePlaidLink({ onSuccess, token: linkToken ?? null });

  useEffect(() => {
    if (ready && !isLinked) {
      open();
    }
  }, [ready, open, isLinked]);

  const {
    data: accessToken,
    isLoading: isLoadingAccessToken,
    error: errorAccessToken,
  } = useExchangePublicToken(publicToken);

  const {
    data: transactions,
    isLoading: isLoadingTransactions,
    error: errorTransactions,
  } = usePlaidGetTransactions(accessToken != null);

  if (errorLinkToken) {
    return <div>Encountered error: {errorLinkToken}</div>;
  }

  if (isLoadingLinkToken || !linkToken) {
    return <div>Loading...</div>;
  }

  return (
    <div>
      <div>Link Token: {linkToken}</div>
      <div>Public Token: {publicToken}</div>
      <div>Access Token: {accessToken}</div>

      <div>
        <h1>Plaid Transactions</h1>
        {errorTransactions ? (
          <div>Error loading plaid transactions: {errorTransactions}</div>
        ) : isLoadingTransactions || !transactions ? (
          <div>Loading plaid transactions...</div>
        ) : (
          <div>
            <table className="w-full">
              <thead>
                <tr>
                  <th className="text-left">Date</th>
                  <th className="text-left">Description</th>
                  <th className="text-left">Account</th>
                  <th className="text-right">Amount</th>
                </tr>
              </thead>
              <tbody>
                {transactions.map((transaction) => (
                  <tr>
                    <td>{transaction.date}</td>
                    <td>{transaction.name}</td>
                    <td>{transaction.account}</td>
                    <td className="text-right font-mono">
                      {transaction.amount}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
}
