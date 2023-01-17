import Link from "next/link";

export default function Sidebar() {
  return (
    <div className="flex flex-col">
      <div>Finance App</div>
      <Link href="/">Home</Link>
      <Link href="/chart">Chart</Link>
      <Link href="/log">Log</Link>
    </div>
  );
}
