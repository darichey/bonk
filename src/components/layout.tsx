import Drawer from "@mui/material/Drawer";
import Link from "./Link";
import Stack from "@mui/material/Stack";
import { Divider } from "@mui/material";

export default function Layout({ children }) {
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
