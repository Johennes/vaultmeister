"use client"

import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

export default function Explorer(): JSX.Element {
  const router = useRouter();

  useEffect(() => {
    console.log('about to start sync');
    invoke("start_sync")
      .then(() => console.log("sync started"))
      .catch(console.error);
  }, []);

  const signOut = () => {
    invoke("sign_out")
      .then(() => router.push("/"))
      .catch(console.error)
  };

  return <div className="flex min-h-screen flex-col items-center justify-center">
    You are logged in!
    <button onClick={signOut}>Sign out</button>
  </div>;
}