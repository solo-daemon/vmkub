import { Inter } from "next/font/google";
import "./globals.css";

import { Web3Modal, Web3ModalProvider } from '../context/Web3Modal'

const inter = Inter({ subsets: ["latin"] });

import  Providers  from "./provider";

export const metadata = {
  title: "Create Next App",
  description: "Generated by create next app",
};

export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <Providers>
          <Web3ModalProvider>{children}</Web3ModalProvider>
          </Providers>
        </body>
    </html>
  );
}
