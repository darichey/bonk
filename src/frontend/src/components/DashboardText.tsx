"use client";

import { useRenderQueryTemplate } from "@/commands";
import { Text } from "../types";

export default function DashboardText({
  text: { template, variables },
}: {
  text: Text;
}) {
  const {
    data: rendered,
    error,
    isLoading,
  } = useRenderQueryTemplate(template, variables);

  if (error) {
    return <div>Encountered error: {error}</div>;
  }

  if (isLoading || !rendered) {
    return <div>Loading...</div>;
  }

  return <div>{rendered}</div>;
}
