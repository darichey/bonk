import { Divider } from "@mui/material";
import Stack from "@mui/material/Stack";
import Link from "./Link";

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
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
  );
}
