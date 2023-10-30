import { useSearchParams } from "react-router-dom";
import MetadataTable from "../components/MetdataTable";

export default function MetadataPage() {
  const [searchParams, _] = useSearchParams();
  const name = searchParams.get("name");

  if (!name) {
    return <div>Error: no metadata name</div>;
  }

  return (
    <div>
      <div>Metadata: {name}</div>
      <MetadataTable name={name} />
    </div>
  );
}
