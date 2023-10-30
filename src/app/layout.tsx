import "./globals.css";
import type { Metadata } from "next";
import { Inter } from "next/font/google";
import NavButton from "./components/NavButton";
import Link from "next/link";
import Image from "next/image";
import financeAppSvg from "../../public/finance-app.svg";
import MetadataSidebar from "./components/MetadataSidebar";
import DashboardSidebar from "./components/DashboardSidebar";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "finance-app",
  description: "finance-app",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="h-screen w-screen">
      <body className={`${inter.className} h-screen w-screen overflow-hidden`}>
        <div className="flex flex-row h-full w-full px-2">
          <div className="flex flex-col border-r-2 w-1/12 px-2">
            <Image
              src={financeAppSvg}
              alt="finance-app logo"
              className="py-4"
            />
            <ul className="mb-5">
              <li>
                <Link href="/">Home</Link>
              </li>
              <li>
                <Link href="/log">Log</Link>
              </li>
              <li>
                <Link href="/query">Query</Link>
              </li>
            </ul>
            <div className="mb-5">
              <MetadataSidebar />
            </div>
            <div className="mb-5">
              <DashboardSidebar />
            </div>
          </div>
          <div className="flex flex-col flex-grow">
            <div className="flex flex-row border-b-2">
              <NavButton dir="back" className="border-r-2" />
              <NavButton dir="forward" className="border-r-2" />
            </div>
            <main className="h-full overflow-y-auto p-4">{children}</main>
          </div>
        </div>
      </body>
    </html>
  );
}
