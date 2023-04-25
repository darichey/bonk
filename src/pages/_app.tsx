import Layout from "../components/layout";
import { DatabaseProvider } from "../db/context";
import { Database } from "../db/database";
import "../style/style.css";
import theme from "../theme";
import { useQuery } from "../util/useQuery";
import { ThemeProvider } from "@mui/material";
import type { AppProps } from "next/app";

export default function MyApp({ Component, pageProps }: AppProps) {
  return (
    <DatabaseProvider>
      <ThemeProvider theme={theme}>
        <Layout>
          <Component {...pageProps} />
        </Layout>
      </ThemeProvider>
    </DatabaseProvider>
  );
}
