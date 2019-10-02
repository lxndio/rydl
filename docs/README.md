# rydl docs

## Modules

rydl is seperated into different modules to make it easy to overlook and easy to maintain. Currently these modules are implemented:

- **Buffer**: This module stores the currently open file in an editor
- **Drawer**: Handles all drawing of things on screen
- **Editor**: The editor itself (i.e. the currently running rydl instance)
- **Handler**: Handles all key input and commands
- **IO**: Used for all IO operations