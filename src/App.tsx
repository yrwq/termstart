import './App.css'
import { useState } from 'react'
import { TerminalDebugger } from './components/TerminalDebugger'
import type { FileSystem } from './filesystem'
import { createEmptyFileSystem } from './filesystem'

function App() {
  const [fs, setFs] = useState<FileSystem>(() => createEmptyFileSystem())

  return (
    <div className="min-h-screen w-screen bg-stone-850 text-white">
      <div className="mx-auto px-6 py-8 space-y-6">
        <TerminalDebugger fs={fs} onFsChange={setFs} />
      </div>
    </div>
  )
}

export default App
