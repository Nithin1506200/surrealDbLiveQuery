"use client";

import { useState } from "react";

export default function ClientCopm() {
  if (typeof window != "undefined") {
    console.log("hello client");
  }

  let x = useState("Fsa");
  return <div>hello client</div>;
}
