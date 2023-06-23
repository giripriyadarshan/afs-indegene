import { useState } from 'react'
import './UploadKeyMessages.css'
import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import getKeyValue from '../utils/getKeyValue'
import StatusList from './StatusList'
import Input from './micro/Input'
import SendButton from './micro/SendButton'


export default function UploadKeyMessages() {
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
            <div className="input-form">
                <div className="input-element">
                    <Input
                        label={"SVN URL"}
                        name={"svn_url"}
                        onChange={(e) => setUrl(e.target.value)}
                        placeholder={"Please enter the svn URL of HTML Folder"}
                        type="url"
                        value={url}
                    />
                </div>

                <div className="input-button">
                    <SendButton
                        disabled={!url.match(urlExpression)}
                        onClick={sendSvnUrl}
                        label={"Go"}
                    />
                </div>
            </div>

            <div className="statuses">
            {kvStatus.status.length > 0 ? <StatusList status={kvStatus.status} title="Upload Status" /> : clickedButton ? <p>Waiting for response...</p> : null}
            {kvStatus.errors.length > 0 ? <StatusList status={kvStatus.errors} title="Errors" /> : clickedButton ? <p>Errors: No errors found</p> : null}
            </div>
        </>
    )
}