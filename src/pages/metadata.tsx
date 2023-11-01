import { useParams } from "react-router-dom";
import MetadataTable from "../components/MetdataTable";

export default function MetadataPage() {
  const { name } = useParams();

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
