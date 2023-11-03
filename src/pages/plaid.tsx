import { useCallback, useEffect, useState } from "react";
import { PlaidLinkOnSuccessMetadata, usePlaidLink } from "react-plaid-link";
import useSWR from "swr";
import { useCreateLinkToken } from "../commands";

export default function PlaidPage() {
  //   const [isLinked, setIsLinked] = useState(false);

  //   const onSuccess = useCallback(() => {
  //     setIsLinked(true);
  //   }, [setIsLinked]);

  //   const token = "link-sandbox-4d39202f-5c7b-4785-8e69-d7ca52b90a4b";

  //   const { open, ready } = usePlaidLink({ onSuccess, token });

  //   useEffect(() => {
  //     if (ready) {
  //       open();
  //     }
  //   }, [ready, open]);

  const {
    data: token,
    isLoading: isLoadingToken,
    error: errorToken,
  } = useCreateLinkToken();

  if (errorToken) {
    return <div>Encountered error: {errorToken}</div>;
  }

  if (isLoadingToken || !token) {
    return <div>Loading...</div>;
  }

  return <div>Got token: {token}</div>;
}
