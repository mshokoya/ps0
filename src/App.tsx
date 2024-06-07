import { Sidebar } from './components/Sidebar'
import { observer } from '@legendapp/state/react'
import { Flex, Theme } from '@radix-ui/themes'
import { TaskView } from './components/TaskQueueBar/TaskView'
import { MetadataAndRecordField } from './components/MetadataRecords'
import { ScrapeField } from './components/ScrapeField'
import { useEffect } from 'react'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import { handleTaskQueueEvent } from './core/io/taskqueue'
import { TQTask } from '.'
import { handleApolloScrapeEndEvent } from './core/io/apollo'

const App = observer(() => {
  useEffect(() => {
    const waitQueue = listen("waitQueue", (event: {payload: TQTask}) => handleTaskQueueEvent(event.payload));
    const processQueue = listen("processQueue", (event: {payload: TQTask}) => handleTaskQueueEvent(event.payload));
    const apolloEvent = listen("apollo", (event: {payload: TQTask}) => handleApolloScrapeEndEvent(event.payload as any));
    // const timeoutQueue = listen("timeoutQueue", (event) => {
    //   console.log(event.payload);
    // });

    // https://github.com/tauri-apps/tauri/discussions/5194
    return () => {
      waitQueue.then((f) => f());
      processQueue.then((f) => f());
      apolloEvent.then((f) => f())
      // timeoutQueue.then((f) => f());
    };
  }, []);

  const handleGet = async () => {
    console.log(
      await invoke("get_accounts", {
        args: [],
      })
    );
  };




  return (
    <Theme accentColor="gray" grayColor="mauve" radius="small" scaling="90%" appearance="dark">
      {/* <button onClick={() => handleGet()}>Get</button> */}
      <a className="ugly-download hidden" />
      <div className="flex relative">
        <Flex direction="column" className=" center h-screen z-0 w-full p-2" gap="3">
          <TaskView />
          <ScrapeField />
          <MetadataAndRecordField />
        </Flex>
        <div>
          <Sidebar />
        </div>
      </div>
    </Theme>
  )
})

export default App
