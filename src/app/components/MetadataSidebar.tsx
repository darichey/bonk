"use client";

import Link from "next/link";
import { useGetMetadataNames } from "../commands";

export default function MetadataSidebar() {
  const { data: metadataNames, isLoading, error } = useGetMetadataNames();

  return (
    <div>
      <div className="text-xs text-gray-500 border-b">Metadata</div>
      {error ? (
        <div>Encountered error: {error}</div>
      ) : isLoading || !metadataNames ? (
        <div>Loading...</div>
      ) : (
        <ul>
          {metadataNames.map((name, i) => (
            <Link href={{ pathname: "/metadata", query: { name } }} key={i}>
              {name}
            </Link>
          ))}
        </ul>
      )}
    </div>
  );
}
