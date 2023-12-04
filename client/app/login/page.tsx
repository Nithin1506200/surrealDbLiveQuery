"use client";

import { FormEvent } from "react";

export default function Login() {
  let onSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    let formData = new FormData(e.target as any);
    let email = formData.get("email");
    let password = formData.get("password");
    alert(email || "" + password || "");
  };
  return (
    <div>
      <form onSubmit={(e) => onSubmit(e)}>
        <input name="email" type="email" className="border-gray-300"></input>
        <input name="password" type="password"></input>
        <button type="submit">submit</button>
      </form>
    </div>
  );
}
