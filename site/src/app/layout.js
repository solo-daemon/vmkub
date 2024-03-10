import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

import { Web3Modal, Web3ModalProvider } from "../context/Web3Modal";
import {NextUIProvider} from "@nextui-org/react";

export const metadata = {
  title: "Create Next App",
  description: "Generated by create next app",
};

export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <body className={inter.className}>

        <Web3ModalProvider>{children}</Web3ModalProvider>
        </body>
    </html>
  );
}
