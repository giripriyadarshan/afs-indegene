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
  const [clickedButton, setClickedButton] = useState(false)
  let kvStatus = getKeyValue(status)

  let urlExpression = new RegExp('^(http|https)://', 'i')

  const sendSvnUrl = () => {
    setStatus([])
    setClickedButton(true)
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
        <input type="url"  placeholder='please enter svn url of HTML folder' id='svn_url' value={url} onInput={
          (e) => {
            setUrl(e.target.value)
          }
        }/>
        <button onClick={sendSvnUrl} disabled={!url.match(urlExpression)} >
            <div class="svg-wrapper-1">
              <div class="svg-wrapper">
                <svg height="24" width="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path d="M0 0h24v24H0z" fill="none"></path>
                  <path d="M1.946 9.315c-.522-.174-.527-.455.01-.634l19.087-6.362c.529-.176.832.12.684.638l-5.454 19.086c-.15.529-.455.547-.679.045L12 14l6-8-8 6-8.054-2.685z" fill="currentColor"></path>
                </svg>
              </div>
            </div>
            <span>Go</span>
          </button>
      </div>
        {kvStatus.length > 0 ? <StatusList status={kvStatus} /> : clickedButton? <p>Waiting for response...</p> : null}
    </>
  )
}

export default App