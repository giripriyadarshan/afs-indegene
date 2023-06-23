import { useState } from 'react'
import './CreateConfigToml.css'
import Input from './micro/Input'
import GenericButton from './micro/GenericButton';

export default function CreateConfigToml() {
    const [url, setUrl] = useState('');
    const [documentLink, setDocumentLink] = useState('');
    const [documentNumber, setDocumentNumber] = useState('');
    return (
        <>
            <h1>Veeva Vault Uploader Config Creator</h1>
            <h2>By Daemons</h2>

            <div className="form">
                <div className="input-element">
                    <Input
                        label={"SVN URL"}
                        name={"svn-url"}
                        onChange={(e) => setUrl(e.target.value)}
                        placeholder={"Please enter the svn URL of HTML Folder"}
                        type={"text"}
                        value={url}
                    />
                </div>

                <div className="input-element">
                    <Input
                        label={"Document Link"}
                        name={"document-link"}
                        onChange={(e) => setDocumentLink(e.target.value)}
                        placeholder={"Please enter the document link"}
                        type={"text"}
                        value={documentLink}
                    />
                </div>

                <div className="input-element">
                    <Input
                        label={"Document Number"}
                        name={"document-number"}
                        onChange={(e) => setDocumentNumber(e.target.value)}
                        placeholder={"Please enter the document number"}
                        type={"number"}
                        value={documentNumber}
                    />
                </div>

                <div className="input-button">
                    <GenericButton label={"Create config.toml"} />
                </div>
            </div>

        </>
    )
}