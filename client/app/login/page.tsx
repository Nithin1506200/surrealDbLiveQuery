"use client";

import Button from "@/components/button/Button";
import { useLogin } from "@/hooks/login.hooks";
import { FormEvent } from "react";

export default function Login() {
  let { token, getToken } = useLogin();
  let onSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    let formData = new FormData(e.target as any);
    let email = formData.get("email")?.toString();
    let password = formData.get("password")?.toString();

    if (email && password) {
      getToken(email, password).then((token) => {
        if (token?.type == "LOADED") {
        }
      });
      alert(email || "" + password || "");
    }
  };
  return (
    <div className=" flex w-full h-full justify-center align-middle ">
      <form
        onSubmit={(e) => onSubmit(e)}
        className="gap-1 flex flex-col justify-center"
      >
        <input
          name="email"
          type="email"
          placeholder="email"
          autoComplete="email"
          required={true}
          className=" text-red-800"
        ></input>
        <input
          name="password"
          type="password"
          required={true}
          // pattern={COMMON_REGEX.password}
          autoComplete="password"
          placeholder="password"
          className="text-red-800"
        ></input>
        <Button type="submit" disabled={token.type === "LOADING"}>
          submit
        </Button>
      </form>
    </div>
  );
}
