"use client";

import { Divider } from "@mui/material";
import Stack from "@mui/material/Stack";
import Link from "next/link";
import Providers from "./providers";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <Providers>
          <Stack
            direction="row"
            spacing={3}
            divider={<Divider orientation="vertical" flexItem />}
          >
            <Stack spacing={1}>
              <Link href="/">Home</Link>
              <Link href="/chart">Chart</Link>
              <Link href="/log">Log</Link>
            </Stack>
            <main>{children}</main>
          </Stack>
        </Providers>
      </body>
    </html>
  );
}
