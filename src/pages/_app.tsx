import Layout from "../components/layout";
import "../style/style.css";
import theme from "../theme";
import { ThemeProvider } from "@mui/material";
import type { AppProps } from "next/app";

export default function MyApp({ Component, pageProps }: AppProps) {
  return (
    <ThemeProvider theme={theme}>
      <Layout>
        <Component {...pageProps} />
      </Layout>
    </ThemeProvider>
  );
}
