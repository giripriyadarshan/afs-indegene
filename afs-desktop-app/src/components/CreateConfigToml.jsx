import { useState } from 'react'
import './CreateConfigToml.css'
import Input from './micro/Input'
import GenericButton from './micro/GenericButton';

export default function CreateConfigToml() {
    const [testInput, setTestInput] = useState('');
    return (
        <>
            <h1>Veeva Vault Uploader Config Creator</h1>
            <h2>By Daemons</h2>

            <div className="form">
                <div className="input-element">
                    <Input
                        label={"SVN URL"}
                        name={"ASDASD"}
                        onChange={(e) => setTestInput(e.target.value)}
                        placeholder={"Please enter the svn URL of HTML Folder"}
                        type={"text"}
                        value={testInput}
                    />
                </div>

                <div className="input-button">
                    <GenericButton label={"Create config.toml"} />
                </div>
            </div>

        </>
    )
}