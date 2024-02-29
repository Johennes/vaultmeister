"use client"

import Button from "@/components/Button";
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

  return <div className="flex min-h-screen flex-col">
    <div className="flex-grow flex flex-col justify-center items-center ">
      You are logged in!
    </div>
    <div className="flex flex-row justify-end p-2">
      <Button isDestructive={true} onClick={signOut}>Sign out</Button>
    </div>
  </div>;
}