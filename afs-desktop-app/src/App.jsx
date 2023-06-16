import { useState, useEffect } from 'react'
import './App.css'
import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'

function App() {
  const [url, setUrl] = useState('')
  const [uuid, setUuid] = useState('')

  const sendSvnUrl = () => {
    invoke('send_url', { svnUrl: url }).then((response) => {
      console.log(response)
      if (uuid !== '') {
        invoke('unsubscribe', { runCode: uuid })
      }
      setUuid(response)
      subscribe(response).then(() => {
        let x = unlisten();
        x.unlisten();
      })
    })
  };

  const subscribe = (response) => {
    invoke('subscribe', { runCode: response }).then((response) => {
      console.log(response)
    })
  };

  return (
    <>
      <h1>Vite + React</h1>
      <div className="input-container">
        <input type="url" id='svn_url' value={url} onInput={
          (e) => {
            setUrl(e.target.value)
          }
        }/>
        <button onClick={sendSvnUrl}>Go</button>
      </div>
      <p>
        {uuid}
      </p>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App

const unlisten = await listen('CurrentMessage', (event) => {
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object
  console.log(event.payload)
})