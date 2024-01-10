import { useRouter } from "next/navigation";

export default function NavButton({
  dir,
  className,
}: {
  dir: "forward" | "back";
  className?: string | undefined;
}) {
  const router = useRouter();

  return (
    <div
      className={className}
      onClick={() => {
        if (dir === "forward") {
          router.forward();
        } else if (dir === "back") {
          router.back();
        }
      }}
    >
      {dir === "forward" ? "->" : "<-"}
    </div>
  );
}
