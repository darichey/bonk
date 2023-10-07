import "./globals.css";
import type { Metadata } from "next";
import { Inter } from "next/font/google";
import NavButton from "./components/NavButton";
import Link from "next/link";
import Image from "next/image";
import financeAppSvg from "../../public/finance-app.svg";
import PageTitle from "./components/PageTitle";

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
    <html lang="en">
      <body className={inter.className}>
        <div className="flex flex-row h-full w-full">
          <div className="flex flex-col border-r-2 w-1/6">
            <Image src={financeAppSvg} alt="finance-app logo" />
            <ul>
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
          </div>
          <div className="flex-auto">
            <div className="flex flex-row border-b-2">
              <NavButton dir="back" className="border-r-2" />
              <NavButton dir="forward" className="border-r-2" />
              <PageTitle />
            </div>
            <main>{children}</main>
          </div>
        </div>
      </body>
    </html>
  );
}
