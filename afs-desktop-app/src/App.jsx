import { useState } from 'react'
import './App.css'
import UploadKeyMessages from './components/UploadKeyMessages'
import CreateConfigToml from './components/CreateConfigToml';
import { Allotment } from 'allotment'

function App() {
  const [configFormToggle, setConfigFormToggle] = useState(false);
  const [paneVisibility, setPaneVisibility] = useState(true);

  return (
    <>
      <button
        className={configFormToggle ? "on" : "off"}
        id='form-toggle'
        onClick={() => setConfigFormToggle(!configFormToggle)}
      >
        Create config.toml
      </button>
      <div className="container">
        {configFormToggle ? <CreateConfigToml /> : <UploadKeyMessages />}
      </div>

      <TopBar />

      <ContentSection>
        {/* bubble up and change the visibility of the pane if it is hidden when clicked on any element on left nav */}
        <LeftNav />

        <TopTabs />

        <Allotment
          snap
          onVisibleChange={(_index, value) => setPaneVisibility(value)}
        >
          <Allotment.Pane minSize={100} visible={paneVisibility}>
            <NavContent />
          </Allotment.Pane>

          <Allotment.Pane minSize={100} snap={false}>
            <Content />
          </Allotment.Pane>
        </Allotment>

      </ContentSection>
    </>
  )
}

export default App