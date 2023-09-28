import { useState } from 'react'
import './CreateConfigToml.css'
import Input from '../micro/Input'
import GenericButton from '../micro/GenericButton';
import { save } from "@tauri-apps/api/dialog";
import { writeTextFile } from '@tauri-apps/api/fs';

export default function CreateConfigToml() {
    const [documentLink, setDocumentLink] = useState('');
    const [documentNumber, setDocumentNumber] = useState('');
    const [sharedDocumentNumber, setSharedDocumentNumber] = useState('');
    const [linkErrorMessage, setLinkErrorMessage] = useState('');
    const [numberErrorMessage, setNumberErrorMessage] = useState('');
    let urlExpression = new RegExp('^(http|https)://', 'i')

    const validateForm = () => {
        let linkValidity = documentLink.match(urlExpression);
        let numberValidity = !isNaN(documentNumber) || !isNaN(sharedDocumentNumber);
        if ( linkValidity && numberValidity ) {
            downloadConfig();
        } 
        if (!linkValidity) {
            setLinkErrorMessage('Please enter a valid document link');
        } else {
            setLinkErrorMessage('');
        }
        if (!numberValidity) {
            setNumberErrorMessage('Please enter a valid document number');
        }
        else {
            setNumberErrorMessage('');
        }
    };


    async function downloadConfig() {

        const filePath = await save({
            filters: [{
                name: 'toml',
                extensions: ['toml']
            }]
        });

        const config = `[vault]\nlink = "${documentLink}"\nbinder_id = "${documentNumber}"\nshared_folder_id = "${sharedDocumentNumber}"`;

        await writeTextFile(filePath, config);
    }


    return (
        <>
            <h1>Veeva Vault Uploader Config Creator</h1>
            <h2>By Daemons</h2>

            <div className="form">

                <div className="input-element">
                    <Input
                        label={"Document Link"}
                        name={"document-link"}
                        onChange={(e) => setDocumentLink(e.target.value)}
                        placeholder={"Please enter the document link"}
                        type={"url"}
                        value={documentLink}
                    />
                </div>

                <div className="input-element">
                    <Input
                        label={"Document Number"}
                        name={"document-number"}
                        onChange={(e) => setDocumentNumber(e.target.value)}
                        placeholder={"Please enter the document number"}
                        type={"text"}
                        value={documentNumber}
                    />
                </div>

                <div className="input-element">
                    <Input
                        label={"Shared Document Number"}
                        name={"shared-document-number"}
                        onChange={(e) => setSharedDocumentNumber(e.target.value)}
                        placeholder={"Please enter the shared document number"}
                        type={"text"}
                        value={sharedDocumentNumber}
                    />
                </div>

                <div className="input-button">
                    <GenericButton label={"Save File"} disabled={false} onClick={validateForm} />
                </div>
            </div>

            <div className="error-messages">
                <p>{linkErrorMessage}</p>
                <p>{numberErrorMessage}</p>
            </div>

        </>
    )
}