# Command Line Interface - Interactive

The interactive mode of the command line interface supports:

* Sub-Commands are suggested
* Auto-Completion
* History
* Reverse Search

## Usage

In order to start an interactive session, use the subcommand client (without any further subcommands):

```shell
$ reactive-graph client
```

You can also start an interactive session to a remote server by specifying the hostname and/or port:

```shell
$ reactive-graph client --hostname=<hostname> --port=<port>
```

## Actions

For all modes:

| Keystroke             | Action                                                                      |
|-----------------------|-----------------------------------------------------------------------------|
| Home                  | Move cursor to the beginning of line                                        |
| End                   | Move cursor to end of line                                                  |
| Left                  | Move cursor one character left                                              |
| Right                 | Move cursor one character right                                             |
| Ctrl-C                | Interrupt/Cancel edition                                                    |
| Ctrl-D, Del           | (if line is _not_ empty) Delete character under cursor                      |
| Ctrl-D                | (if line _is_ empty) End of File                                            |
| Ctrl-J, Ctrl-M, Enter | Finish the line entry                                                       |
| Ctrl-R                | Reverse Search history (Ctrl-S forward, Ctrl-G cancel)                      |
| Ctrl-T                | Transpose previous character with current character                         |
| Ctrl-U                | Delete from start of line to cursor                                         |
| Ctrl-V                | Insert any special character without performing its associated action (#65) |
| Ctrl-W                | Delete word leading up to cursor (using white space as a word boundary)     |
| Ctrl-Y                | Paste from Yank buffer                                                      |
| Ctrl-Z                | Suspend (Unix only)                                                         |
| Ctrl-\_               | Undo                                                                        |

### Emacs mode (default mode)

| Keystroke         | Action                                                                                           |
|-------------------|--------------------------------------------------------------------------------------------------|
| Ctrl-A, Home      | Move cursor to the beginning of line                                                             |
| Ctrl-B, Left      | Move cursor one character left                                                                   |
| Ctrl-E, End       | Move cursor to end of line                                                                       |
| Ctrl-F, Right     | Move cursor one character right                                                                  |
| Ctrl-H, Backspace | Delete character before cursor                                                                   |
| Ctrl-I, Tab       | Next completion                                                                                  |
| Ctrl-K            | Delete from cursor to end of line                                                                |
| Ctrl-L            | Clear screen                                                                                     |
| Ctrl-N, Down      | Next match from history                                                                          |
| Ctrl-P, Up        | Previous match from history                                                                      |
| Ctrl-X Ctrl-U     | Undo                                                                                             |
| Ctrl-Y            | Paste from Yank buffer (Meta-Y to paste next yank instead)                                       |
| Meta-<            | Move to first entry in history                                                                   |
| Meta->            | Move to last entry in history                                                                    |
| Meta-B, Alt-Left  | Move cursor to previous word                                                                     |
| Meta-C            | Capitalize the current word                                                                      |
| Meta-D            | Delete forwards one word                                                                         |
| Meta-F, Alt-Right | Move cursor to next word                                                                         |
| Meta-L            | Lower-case the next word                                                                         |
| Meta-T            | Transpose words                                                                                  |
| Meta-U            | Upper-case the next word                                                                         |
| Meta-Y            | See Ctrl-Y                                                                                       |
| Meta-Backspace    | Kill from the start of the current word, or, if between words, to the start of the previous word |
| Meta-0, 1, ..., - | Specify the digit to the argument. `–` starts a negative argument.                               |

[Readline Emacs Editing Mode Cheat Sheet](http://www.catonmat.net/download/readline-emacs-editing-mode-cheat-sheet.pdf)

### vi command mode

| Keystroke            | Action                                                                      |
|----------------------|-----------------------------------------------------------------------------|
| $, End               | Move cursor to end of line                                                  |
| .                    | Redo the last text modification                                             |
| ;                    | Redo the last character finding command                                     |
| ,                    | Redo the last character finding command in opposite direction               |
| 0, Home              | Move cursor to the beginning of line                                        |
| ^                    | Move to the first non-blank character of line                               |
| a                    | Insert after cursor                                                         |
| A                    | Insert at the end of line                                                   |
| b                    | Move one word or token left                                                 |
| B                    | Move one non-blank word left                                                |
| c<movement>          | Change text of a movement command                                           |
| C                    | Change text to the end of line (equivalent to c$)                           |
| d<movement>          | Delete text of a movement command                                           |
| D, Ctrl-K            | Delete to the end of the line                                               |
| e                    | Move to the end of the current word                                         |
| E                    | Move to the end of the current non-blank word                               |
| f<char>              | Move right to the next occurrence of `char`                                 |
| F<char>              | Move left to the previous occurrence of `char`                              |
| h, Ctrl-H, Backspace | Move one character left                                                     |
| l, Space             | Move one character right                                                    |
| Ctrl-L               | Clear screen                                                                |
| i                    | Insert before cursor                                                        |
| I                    | Insert at the beginning of line                                             |
| +, j, Ctrl-N         | Move forward one command in history                                         |
| -, k, Ctrl-P         | Move backward one command in history                                        |
| p                    | Insert the yanked text at the cursor (paste)                                |
| P                    | Insert the yanked text before the cursor                                    |
| r                    | Replaces a single character under the cursor (without leaving command mode) |
| s                    | Delete a single character under the cursor and enter input mode             |
| S                    | Change current line (equivalent to 0c$)                                     |
| t<char>              | Move right to the next occurrence of `char`, then one char backward         |
| T<char>              | Move left to the previous occurrence of `char`, then one char forward       |
| u                    | Undo                                                                        |
| w                    | Move one word or token right                                                |
| W                    | Move one non-blank word right                                               |
| x                    | Delete a single character under the cursor                                  |
| X                    | Delete a character before the cursor                                        |
| y<movement>          | Yank a movement into buffer (copy)                                          |

### vi insert mode

| Keystroke         | Action                         |
|-------------------|--------------------------------|
| Ctrl-H, Backspace | Delete character before cursor |
| Ctrl-I, Tab       | Next completion                |
| Esc               | Switch to command mode         |

[Readline vi Editing Mode Cheat Sheet](http://www.catonmat.net/download/bash-vi-editing-mode-cheat-sheet.pdf)

[Terminal codes (ANSI/VT100)](http://wiki.bash-hackers.org/scripting/terminalcodes)
