"use client";

import { NextUIProvider } from "@nextui-org/react";

export default function Providers({ children, session }) {
  return (
    <NextUIProvider>
            <main className="dark text-foreground bg-background">
        {children}
        </main>
    </NextUIProvider>
  );
}