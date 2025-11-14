import type { Metadata } from "next";
import "./globals.css";
import Layout from "@/components/Layout";

export const metadata: Metadata = {
  metadataBase: new URL(process.env.NEXT_PUBLIC_SITE_URL || "http://localhost:3000"),
  title: {
    default: "Developer Blog",
    template: "%s | Developer Blog",
  },
  description: "A developer blog sharing knowledge and insights. I started this blog to deepen my knowledge and slow things down in a world moving at a frightening pace.",
  keywords: ["blog", "developer", "programming", "web development", "technology", "software development", "coding", "tutorials"],
  authors: [{ name: "Developer" }],
  creator: "Developer",
  publisher: "Developer",
  formatDetection: {
    email: false,
    address: false,
    telephone: false,
  },
  openGraph: {
    type: "website",
    locale: "en_US",
    title: "Developer Blog",
    description: "A developer blog sharing knowledge and insights. I started this blog to deepen my knowledge and slow things down in a world moving at a frightening pace.",
    siteName: "Developer Blog",
  },
  twitter: {
    card: "summary_large_image",
    title: "Developer Blog",
    description: "A developer blog sharing knowledge and insights. I started this blog to deepen my knowledge and slow things down in a world moving at a frightening pace.",
    creator: "@developer",
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      "max-video-preview": -1,
      "max-image-preview": "large",
      "max-snippet": -1,
    },
  },
  alternates: {
    canonical: "/",
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
        <html lang="en" className="dark">
          <body
            className="antialiased bg-[#0f0f0f] text-white"
          >
        <Layout>
          {children}
        </Layout>
      </body>
    </html>
  );
}
