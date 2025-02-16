# Those things will eventually be done, but for now they are just cluttering TODO.md

### Operations
- [ ] Add a new task
- [ ] Remove a task
- [ ] Change name of a task
- [ ] Check/uncheck some box
- [ ] Add/remove a board

### Then, web interface (web edit -> file update)
- [ ] Render UI elements embedded in Markdown using `leptos-markdown`'s [custom
      components](https://github.com/rambip/leptos-markdown/blob/main/examples/custom_component/src/main.rs))
      (Also note the same author made [dioxus-markdown](https://github.com/rambip/dioxus-markdown))
      (Note too that the author stated that it will move to [rust-web-markdown](https://github.com/rambip/rust-web-markdown),
      which right now contains a framework agnostic implementation)
- [ ] Add button to create lists
- [ ] Add button to create tasks
- [ ] Button to remove lists
- [ ] Button to remove tasks
- [ ] Add editbox to edit a task name

### Undo functionality
- [ ] Save previous versions in .history directory, like the [local history](https://marketplace.visualstudio.com/items?itemName=xyz.local-history) vscode extension

### Stretch goal: make it bidirectional (file edit -> web update)
- [ ] Watch file
- [ ] Update web interface when file changes

### And maybe import from other formats?
- [ ] Maybe the format used by [nullboard](https://nullboard.io/preview) or something
