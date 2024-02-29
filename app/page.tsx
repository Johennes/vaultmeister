"use client"

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'
import Input from '@/components/Input';
import { useRouter } from 'next/navigation';

class Status {
  static initial = new Status(false, null);
  static loading = new Status(true, null);
  static error = (error: string) => new Status(false, error);

  constructor(
    readonly isLoading: boolean,
    readonly error: string | null
  ) {}
}

export default function Home(): JSX.Element {
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
  
  const [status, setStatus] = useState(Status.initial);
  const [username, setUsername] = useState("vaultmeister");
  const [password, setPassword] = useState("vaultmeister");

  const router = useRouter();
  
  const signIn = () => {
    setStatus(Status.loading);
    invoke("sign_in", { username, password })
      .then(() => router.push("/explorer"))
      .catch(error => setStatus(Status.error(error)))
  };

  return (
    <main className="flex min-h-screen flex-col items-center justify-start pt-20 gap-10">
      <div>
        <h2 className="text-2xl font-semibold">
          Vaultmeister
        </h2>
        <div className="text-xs text-right">v{version}</div>
      </div>
      <div>
        <Input type="text" placeholder="Home server" value={homeServer} disabled={true} />
        <Input type="text" placeholder="Username" value={username} disabled={status.isLoading} onChange={e => setUsername(e.target.value)} />
        <Input type="password" placeholder="Password" value={password} disabled={status.isLoading} onChange={e => setPassword(e.target.value)} />
        <input
          type="submit"
          className="block w-full rounded-md border-0 py-1.5 px-2 bg-green-700 hover:bg-green-800 text-white font-bold text-sm mb-1"
          value="Sign in"
          disabled={status.isLoading}
          onClick={signIn} />
      </div>
      {status.error && <div className="text-red-600 px-10 text-center">{status.error}123</div>}
    </main>
  );
}
