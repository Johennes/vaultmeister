import { ButtonHTMLAttributes } from "react"

export interface IButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  isDestructive?: boolean
}

export default function Button(props: IButtonProps): JSX.Element {
  return <button
    {...props}
    className={(props.className ?? "") + " block rounded-md border-0 py-1.5 px-2 text-white font-bold text-sm mb-1 "
      + (props.isDestructive ? "bg-red-700 hover:bg-red-800" : "bg-green-700 hover:bg-green-800")}
  >
    {props.children}
  </button>
}
