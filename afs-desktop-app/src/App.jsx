import { useState } from 'react'
import './App.css'
import UploadKeyMessages from './components/content/UploadKeyMessages'
import CreateConfigToml from './components/content/CreateConfigToml';
import { Allotment } from 'allotment'
import LeftNav from './components/left-nav/LeftNav';
import LeftNavTask from './components/left-nav/LeftNavTask';
import ContentSection from './components/content/ContentSection';

function App() {
  const [configFormToggle, setConfigFormToggle] = useState(false);
  const [paneVisibility, setPaneVisibility] = useState(true);

  const [activeTask, setActiveTask] = useState('veeva');

  return (
    <>
      {/* <button
        className={configFormToggle ? "on" : "off"}
        id='form-toggle'
        onClick={() => setConfigFormToggle(!configFormToggle)}
      >
        Create config.toml
      </button>
      <div className="container">
        {configFormToggle ? <CreateConfigToml /> : <UploadKeyMessages />}
      </div> */}

      {/* <TopBar /> */}

      <ContentSection>
        {/* bubble up and change the visibility of the pane if it is hidden when clicked on any element on left nav */}
        <LeftNav > 
          <LeftNavTask task='veeva' navClick={() => setActiveTask("veeva")} activeTask={activeTask} />
          <LeftNavTask task='comingSoon' navClick={() => setActiveTask("comingSoon")} activeTask={activeTask} />
        </LeftNav>

        {/* <TopTabs /> */}

        <Allotment
          snap
          onVisibleChange={(_index, value) => setPaneVisibility(value)}
        >
          <Allotment.Pane minSize={100} visible={paneVisibility}>
            {/* <NavContent /> */}
          </Allotment.Pane>

          <Allotment.Pane minSize={100} snap={false}>
            {/* <Content /> */}
          </Allotment.Pane>
        </Allotment>

      </ContentSection>
    </>
  )
}

export default App