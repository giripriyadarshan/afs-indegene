import { useState, useEffect } from 'react'
import './App.css'
import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import getKeyValue from './utils/getKeyValue'
import StatusList from './components/StatusList'

function App() {
  const [url, setUrl] = useState('')
  const [uuid, setUuid] = useState('')
  const [status, setStatus] = useState([])
  let kvStatus = getKeyValue(status)

  let urlExpression = new RegExp('^(http|https)://', 'i')

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
    })
  };

  const subscribe = (response) => {
    invoke('subscribe', { runCode: response })
  };

  async function listenData() {
    await listen('CurrentMessage', (event) => {
      setStatus(status => [...status, event.payload])
    })
  }

  return (
    <>
      <h1>Veeva Vault Uploader</h1>
      <h2>By Daemons</h2>
      <div className="input-container">
        <input type="url"  placeholder='please enter svn HTML folder url' id='svn_url' value={url} onInput={
          (e) => {
            setUrl(e.target.value)
          }
        }/>
        <button onClick={sendSvnUrl} disabled={!url.match(urlExpression)} >Go</button>
      </div>
        {kvStatus.length > 0 ? <StatusList status={kvStatus} /> : null}
    </>
  )
}

export default App