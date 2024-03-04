"use client"

import Button from "@/components/Button";
import Input from "@/components/Input";
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

export interface IRoom {
  id: string | undefined
  name: string | undefined
}

export default function Explorer(): JSX.Element {
  const router = useRouter();

  const [rooms, setRooms] = useState<IRoom[]>([]);
  const [selection, setSelection] = useState<IRoom | undefined>(undefined);

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

  const showRoomCreationForm = () => {
    setSelection({} as IRoom);
  };

  const createRoom = (room: IRoom) => {
    invoke("create_room")
      .then(console.log)
      .catch(console.error)
  };

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
        {selection && <div>
            <div>
              Name <Input/>
            </div>
            {!selection.id && <Button onClick={ev => createRoom(selection)}>Save</Button>}
          </div>}
      </div>
    </main>
    <footer className="flex flex-row justify-between px-4 py-2">
      <Button onClick={showRoomCreationForm}>New</Button>
      <Button isDestructive={true} onClick={signOut}>Sign out</Button>
    </footer>
  </div>;
}