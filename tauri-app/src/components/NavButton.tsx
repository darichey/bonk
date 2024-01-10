import { useNavigate } from "react-router-dom";

export default function NavButton({
  dir,
  className,
}: {
  dir: "forward" | "back";
  className?: string | undefined;
}) {
  const navigate = useNavigate();

  return (
    <div
      className={className}
      onClick={() => {
        if (dir === "forward") {
          navigate(1);
        } else if (dir === "back") {
          navigate(-1);
        }
      }}
    >
      {dir === "forward" ? "->" : "<-"}
    </div>
  );
}
