# termstart

<p align="center">
  <img width="460" height="300" src="https://github.com/user-attachments/assets/c6d5cb88-29e5-4c7a-88e7-c1f3b42caf5e">
</p>

terminal themed bookmark manager for web browsers

behaves like a small shell:
- navigate directories
- create/delete/move files and folders
- store bookmark URLs in files
- open bookmark files in a new tab
- persist everything to `localStorage`

## features

- shell like commands (`cd`, `ls`, `mkdir`, `rm`, `mv`, `cat`, `tree`, etc.)
- `man <command>` help pages
- `help` to list commands
- theme switching (`theme`, `theme list`, `theme <name>`)
- tab completion for commands, paths, themes, and `man` pages
- keyboard friendly input/history behavior

## commands

| command | description |
| --- | --- |
| `help` | list commands |
| `man <command>` | show help and usage for a command |
| `pwd` | print current directory |
| `ls [path]` | list directory contents |
| `cd [path]` | change current directory |
| `mkdir [-p] <path>` | create directories |
| `touch <path> <url>` | create bookmark file with URL |
| `cat <path>` | print bookmark URL |
| `open <path>` | open bookmark URL in a new tab |
| `mv <src> <dest>` | move or rename file/directory |
| `rm [-r] <path>` | remove file or directory (`-r` for directories) |
| `tree [path]` | print directory tree |
| `theme [name\|list]` | show/set/list themes |
| `clear` | clear terminal |

## shortcuts

- `Tab`: autocomplete
- `ArrowUp` / `ArrowDown`: command history
- `Ctrl/Cmd + A`: move cursor to start
- `Ctrl/Cmd + E`: move cursor to end
- `Ctrl/Cmd + L`: clear output
- `Esc`: release input focus lock
