"use client";

import { usePathname } from "next/navigation";

export default function PageTitle() {
  const pathname = usePathname();

  return <div>{pathname}</div>;
}
