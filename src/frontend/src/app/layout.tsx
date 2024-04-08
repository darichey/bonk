import { Inter } from "next/font/google";
import "./globals.css";
import Link from "next/link";
import DashboardSidebar from "@/components/DashboardSidebar";
import NavButton from "@/components/NavButton";
import Image from "next/image";
import financeAppSvg from "../../public/finance-app.svg";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="w-full h-full">
      <body className={`w-full h-full ${inter.className}`}>
        <div className="flex flex-row h-full w-full px-2">
          <div className="flex flex-col border-r-2 w-1/12 px-2">
            <Image
              src={financeAppSvg}
              alt="finance-app logo"
              className="py-4"
              width={400}
            />
            <ul className="mb-5">
              <li>
                <Link href="/">🏠 Home</Link>
              </li>
              <li>
                <Link href="/log">📝 Log</Link>
              </li>
              <li>
                <Link href="/query">❓ Query</Link>
              </li>
              <li>
                <Link href="/chat">✨ Chat</Link>
              </li>
            </ul>
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
