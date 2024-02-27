import { InputHTMLAttributes } from "react";

export interface IInputProps extends InputHTMLAttributes<HTMLInputElement> {}

export default function Input(props: IInputProps) {
  return (
    <input
      type="text"
      className="block w-full rounded-md border-0 py-1.5 px-2 text-gray-900 disabled:text-gray-400 disabled:bg-gray-200 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 text-sm mb-1"
      {...props}
    />
  );
}
