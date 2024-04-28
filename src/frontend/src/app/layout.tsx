import { Inter } from "next/font/google";
import "./globals.css";
import Link from "next/link";
import DashboardSidebar from "@/components/DashboardSidebar";
import Image from "next/image";
import bonkSvg from "../../public/bonk.svg";

const inter = Inter({ subsets: ["latin"] });
const demoMode = process.env.BONK_DEMO_MODE ? true : false;

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="w-full h-full">
      <body className={`w-full h-full ${inter.className}`}>
        {demoMode ? (
          <div className="bg-yellow-100 text-center p-1">
            Demo mode: fake data
          </div>
        ) : (
          <></>
        )}

        <div className="flex flex-row h-full w-full px-2">
          <div className="flex flex-col border-r-2 w-1/12 px-2">
            <Image src={bonkSvg} alt="bonk logo" className="py-4" width={100} />
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
            <main className="h-full overflow-y-auto p-4">{children}</main>
          </div>
        </div>
      </body>
    </html>
  );
}
