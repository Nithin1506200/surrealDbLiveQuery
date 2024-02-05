import { ButtonHTMLAttributes, DetailedHTMLProps } from "react";

export default function Button(
  props: DetailedHTMLProps<
    ButtonHTMLAttributes<HTMLButtonElement>,
    HTMLButtonElement
  >
) {
  return (
    <button
      {...props}
      className=" border hover:bg-slate-200 hover:text-black hover:border-cyan-400 cursor-pointer"
    >
      {props.children}{" "}
    </button>
  );
}

Button.Primary = {};

Button.Secondary = {};
