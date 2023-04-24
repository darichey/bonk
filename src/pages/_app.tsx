import "../style/style.css";
import type { AppProps } from "next/app";
import Layout from "../components/layout";
import { ThemeProvider } from "@mui/material";
import theme from "../theme";

export default function MyApp({ Component, pageProps }: AppProps) {
  return (
    <ThemeProvider theme={theme}>
      <Layout>
        <Component {...pageProps} />
      </Layout>
    </ThemeProvider>
  );
}
