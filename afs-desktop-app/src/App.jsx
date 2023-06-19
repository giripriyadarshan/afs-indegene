import { useState, useEffect } from 'react'
import './App.css'
import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import getKeyValue from './utils/getKeyValue'

function App() {
  const [url, setUrl] = useState('')
  const [uuid, setUuid] = useState('')
  const [status, setStatus] = useState([])
  let kvStatus = getKeyValue(status)

  const sendSvnUrl = () => {
    setStatus([])
    invoke('send_url', { svnUrl: url }).then((response) => {
      console.log(response)
      if (uuid !== '') {
        invoke('unsubscribe', { runCode: uuid })
      }
      setUuid(response)
      subscribe(response)

      listenData()
      setTimeout(() => {
        console.log('listen call')
      }, 10000)
    })
  };

  const subscribe = (response) => {
    console.log(invoke('subscribe', { runCode: response }))
  };

  async function listenData() {
    await listen('CurrentMessage', (event) => {
      console.log("listen event")
      setStatus(status => [...status, event.payload])
    })
  }
  console.log(kvStatus)

  return (
    <>
      <div className="input-container">
        <input type="url" id='svn_url' value={url} onInput={
          (e) => {
            setUrl(e.target.value)
          }
        }/>
        <button onClick={sendSvnUrl} placeholder='please enter svn HTML url'>Go</button>

        <div className="status-container">
          <ul>
            {kvStatus.length > 0 && kvStatus.map((item, index) => 
                <li key={index}>{item.key} : {item.value}</li>
            )}
          </ul>
          </div>
      </div>
    </>
  )
}

export default App