import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import "./index.css";
import DashboardPage from "./pages/dashboard.tsx";
import HomePage from "./pages/home.tsx";
import LogPage from "./pages/log.tsx";
import MetadataPage from "./pages/metadata.tsx";
import QueryPage from "./pages/query.tsx";
import ImportPage from "./pages/import.tsx";
import { SWRConfig } from "swr";

const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
    children: [
      {
        path: "/",
        element: <HomePage />,
      },
      {
        path: "/dashboard/:name",
        element: <DashboardPage />,
      },
      {
        path: "/import",
        element: <ImportPage />,
      },
      {
        path: "/log",
        element: <LogPage />,
      },
      {
        path: "/metadata/:name",
        element: <MetadataPage />,
      },
      {
        path: "/query",
        element: <QueryPage />,
      },
    ],
  },
]);

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <SWRConfig
      value={{
        revalidateOnFocus: false,
      }}
    >
      <RouterProvider router={router} />
    </SWRConfig>
  </React.StrictMode>
);
