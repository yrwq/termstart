import '@/App.css'
import { useEffect, useState } from 'react'
import { Terminal } from '@/components/Terminal'
import type { FileSystem } from '@/filesystem'
import { createEmptyOrFallback, serializeFileSystem } from '@/filesystem'

const STORAGE_KEY = 'terminal-bookmark-manager:filesystem'
const THEME_KEY = 'terminal-bookmark-manager:theme'

function App() {
  const [storageError, setStorageError] = useState<string | null>(null)
  const [theme, setTheme] = useState(() => {
    try {
      const stored = window.localStorage.getItem(THEME_KEY)
      return stored ?? 'amber'
    } catch (error) {
      console.error(error)
      return 'amber'
    }
  })
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

  useEffect(() => {
    document.documentElement.dataset.theme = theme
    try {
      window.localStorage.setItem(THEME_KEY, theme)
    } catch (error) {
      console.error(error)
    }
  }, [theme])

  return (
    <div className="min-h-screen w-screen terminal-page">
      <div className="terminal-container">
        {storageError && (
          <div className="terminal-warning">
            {storageError}
          </div>
        )}
        <Terminal
          fs={fs}
          onFsChange={setFs}
          theme={theme}
          onThemeChange={setTheme}
        />
      </div>
    </div>
  )
}

export default App
