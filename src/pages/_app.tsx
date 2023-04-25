import { ThemeProvider } from "@mui/material";
import type { AppProps } from "next/app";
import Layout from "../components/layout";
import { DatabaseProvider } from "../db/context";
import "../style/style.css";
import theme from "../theme";

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
