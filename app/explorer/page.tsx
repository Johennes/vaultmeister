"use client"

import Button from "@/components/Button";
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

export interface IRoom {
  id: string
  name: string
}

export default function Explorer(): JSX.Element {
  const router = useRouter();

  const [rooms, setRooms] = useState<IRoom[]>([]);

  const fetchRooms = () => {
    invoke("get_rooms")
      .then(rooms => setRooms(rooms as IRoom[]))
      .catch(console.error);
  };

  useEffect(() => {
    invoke("start_sync")
      .then(fetchRooms)
      .catch(console.error);
  }, []);

  const signOut = () => {
    invoke("sign_out")
      .then(() => router.push("/"))
      .catch(console.error)
  };

  return <div className="flex min-h-screen flex-col">
    <main className="flex-grow flex flex-row pt-8 px-4">
      <div className="p-2 bg-white rounded-sm border-green-400 border min-w-40">
        {rooms.map(room => <div key={room.id}>{room.name}</div>)}
      </div>
      <div className="flex-grow flex flex-col justify-center items-center">
        Room details
      </div>
    </main>
    <div className="flex flex-row justify-end p-2">
      <Button isDestructive={true} onClick={signOut}>Sign out</Button>
    </div>
  </div>;
}