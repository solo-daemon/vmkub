"use client";

import { NextUIProvider } from "@nextui-org/react";

export default function Providers({ children, session }) {
  return (
    <NextUIProvider>
        {children}
    </NextUIProvider>
  );
}