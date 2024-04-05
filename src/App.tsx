import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import { WebviewWindow } from '@tauri-apps/api/window'

function App() {
  const [accessToken, setAccessToken] = useState('');
  const [idToken, setIdToken] = useState('');
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

  async function login() {
    await invoke('valorant_login');

    const mainWindow = WebviewWindow.getByLabel('external');

    mainWindow?.listen('on_nav', (event) => {
      const url = event.payload as string;
      if(url.startsWith('https://playvalorant.com')) {
        const paramStr = url.substring(url.indexOf('#a') + 1);
        const params = new URLSearchParams(paramStr);
        
        setAccessToken(params.get('access_token') || '');
        setIdToken(params.get('id_token') || '');
        
        // mainWindow.close();
      }
    })

    mainWindow?.listen('valorant_loggin', (event) => {
      const {username, password} = event.payload as {username: string, password: string};
      setUsername(username);
      setPassword(password);
    })
  }

  return (
    <div className="container">
      <h1>Demo</h1>

      <p>username: {username}</p>
      <p>password: {password}</p>
      <p>access token: {accessToken}</p>
      <p>id token: {idToken}</p>

      <button onClick={login}>Login</button>

      <p></p>
    </div>
  );
}

export default App;
