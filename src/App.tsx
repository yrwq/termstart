import '@/App.css'
import { useEffect, useState } from 'react'
import { Terminal } from '@/components/Terminal'
import type { FileSystem } from '@/filesystem'
import { createEmptyOrFallback, serializeFileSystem } from '@/filesystem'

const STORAGE_KEY = 'terminal-bookmark-manager:filesystem'

function App() {
  const [storageError, setStorageError] = useState<string | null>(null)
  const [fs, setFs] = useState<FileSystem>(() => {
    try {
      const raw = window.localStorage.getItem(STORAGE_KEY)
      return createEmptyOrFallback(raw)
    } catch (error) {
      console.error(error)
      return createEmptyOrFallback(null)
    }
  })

  useEffect(() => {
    try {
      const serialized = serializeFileSystem(fs)
      window.localStorage.setItem(STORAGE_KEY, serialized)
      if (storageError) setStorageError(null)
    } catch (error) {
      console.error(error)
      setStorageError('Local storage is unavailable. Changes will not persist.')
    }
  }, [fs, storageError])

  return (
    <div className="min-h-screen w-screen bg-stone-950 text-white">
      <div className="mx-auto px-6 py-8 space-y-6">
        {storageError && (
          <div className="rounded-lg px-4 py-3 text-sm text-amber-100">
            {storageError}
          </div>
        )}
        <Terminal fs={fs} onFsChange={setFs} />
      </div>
    </div>
  )
}

export default App
