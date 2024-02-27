'use client'

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'
import Input from '@/components/input';

export default function Home() {
  const [version, setVersion] = useState("");
  const [homeServer, setHomeServer] = useState("");

  useEffect(() => {
    invoke<string>("version")
      .then(setVersion)
      .catch(console.error)
    invoke<string>("homeserver")
      .then(setHomeServer)
      .catch(console.error)
  }, [])

  const [username, setUsername] = useState("vaultmeister");
  const [password, setPassword] = useState("vaultmeister");

  const signIn = () => {
    invoke("sign_in", { username, password })
      .then(x => alert(x))
      .catch(e => alert(e))
  };

  return (
    <main className="flex min-h-screen flex-col items-center justify-center gap-10">
      <div>
        <h2 className="text-2xl font-semibold">
          Vaultmeister
        </h2>
        <div className="text-xs text-right">v{version}</div>
      </div>
      <div>
        <Input type="text" placeholder="Home server" value={homeServer} disabled={true} />
        <Input type="text" placeholder="Username" value={username} onChange={e => setUsername(e.target.value)} />
        <Input type="password" placeholder="Password" value={password} onChange={e => setPassword(e.target.value)} />
        <input
          type="submit"
          className="block w-full rounded-md border-0 py-1.5 px-2 bg-green-700 hover:bg-green-800 text-white font-bold text-sm mb-1"
          value="Sign in"
          onClick={signIn} />
      </div>
    </main>
  );
}
