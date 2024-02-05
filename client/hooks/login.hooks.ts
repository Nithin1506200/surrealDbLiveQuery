import { ENV } from "@/config";
import { apiResult, result } from "@/types/result";
import sessionStorageUtils from "@/utils/sessionStorage.utils";
import axios, { AxiosResponse } from "axios";
import { useEffect, useState } from "react";
type login = {
  token: string;
};
export async function loginApi(email: string, password: string) {
  let result: AxiosResponse<login, any> = await axios.post(
    "http://127.0.0.1:8001/api/login",
    {
      email,
      password,
    },
    {
      headers: {
        "Content-Type": "application/json;charset=UTF-8",
      },
    }
  );
  let res: result<login, string>;
  switch (result.status) {
    case 200:
      let data = result.data;
      res = { type: "OK", data: data };
      break;
    default:
      res = { type: "ERR", data: "Login Failed" };
      break;
  }
  return res;
}
export function useLogin() {
  let [token, setToken] = useState<apiResult<login, string>>({ type: "INIT" });
  let getToken = async (email: string, password: string) => {
    setToken({ type: "LOADING" });
    let result = await loginApi(email, password);
    let r = token;
    switch (result.type) {
      case "OK":
        token = { type: "LOADED", data: result.data };
        sessionStorageUtils.setLoginToken(result.data.token);
        break;
      case "ERR":
        token = { type: "ERROR", err: result.data };
        break;
    }
    setToken(token);
    return token;
  };
  return {
    getToken,
    token,
  };
}
/**
 * get through token
 */
export function useGetUserDetail() {}
