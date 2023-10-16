import MetadataTable from "../components/MetdataTable";

export default function MetadataPage({
  searchParams,
}: {
  searchParams: { [key: string]: string | string[] | undefined };
}) {
  const name = searchParams["name"] as string; // TODO

  return (
    <div>
      <div>Metadata: {name}</div>
      <MetadataTable name={name} />
    </div>
  );
}
