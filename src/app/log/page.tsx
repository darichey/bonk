import { Metadata } from "next";
import TransactionLog from "../components/TransactionLog";

export const metadata: Metadata = {
  title: "Log",
};

export default function LogPage() {
  return <TransactionLog />;
}
