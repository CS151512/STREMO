import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "STREMO - Stream Everything",
  description: "A professional streaming platform landing page.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="h-full antialiased dark">
      <body className="h-full bg-[#131315] text-[#f4f4f5]">
        {children}
      </body>
    </html>
  );
}
