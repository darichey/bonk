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
        path: "/dashboard",
        element: <DashboardPage />,
      },
      {
        path: "/log",
        element: <LogPage />,
      },
      {
        path: "/metadata",
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
    <RouterProvider router={router} />
  </React.StrictMode>
);
