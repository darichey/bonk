import { Metadata } from "next";
import Query from "../components/Query";

export const metadata: Metadata = {
  title: "Query",
};

export default function QueryPage() {
  return <Query />;
}
