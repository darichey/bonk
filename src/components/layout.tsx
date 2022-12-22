import Sidebar from "./Sidebar";

export default function Layout({ children }) {
  return (
    <div className="flex gap-4">
      <Sidebar />
      <main>{children}</main>
    </div>
  );
}
