import { useState} from 'react'
import './App.css'
import UploadKeyMessages from './components/UploadKeyMessages'
import CreateConfigToml from './components/CreateConfigToml';

function App() {
  const [configFormToggle, setConfigFormToggle] = useState(false);

  return (
    <>
      <button 
      className={configFormToggle? "on" : "off"} 
      id='form-toggle'
      onClick={() => setConfigFormToggle(!configFormToggle)}
      >
        Create config.toml
      </button>
      <div className="container">
        {configFormToggle? <CreateConfigToml/> : <UploadKeyMessages />}
      </div>
    </>
  )
}

export default App