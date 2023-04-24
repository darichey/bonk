import Link from "./Link";
import { Divider } from "@mui/material";
import Drawer from "@mui/material/Drawer";
import Stack from "@mui/material/Stack";

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
