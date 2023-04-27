"use client";

import { ThemeProvider } from "@mui/material";
import theme from "../theme";

export default function Providers({ children }: { children: React.ReactNode }) {
  return <ThemeProvider theme={theme}>{children}</ThemeProvider>;
}
