import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Stellar Identity",
  description: "Onchain Identity & Reputation on Stellar",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
