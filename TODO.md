# markanban - markdown kanban board



This is     a test.



### First, test waters
- [x] Pretty print markdown without changing it; see if it loses information
      (unfortunately, markdown-ast will not preserve the original markdown: it will strip two or more empty lines, and will escape the [ ]s,
      among other things. maybe markedit will fare better? or better, pulldown-cmark-to-cmark)
- [ ] Try configuring pulldown-cmark to add the required extension (tasklist) and use events_to_ast rather than markdown_to_ast


### Map from markdown-ast into my own ast and vice versa
- [ ] ...

- [ ] Add a new task
- [ ] Remove a task
- [ ] Change name of a task
- [ ] Check/uncheck some box
- [ ] Add/remove a board

### Then, web interface (web edit -> file update)
- [ ] Leptos hello world
- [ ] Render markdown myself (so I can edit it easily without another lib)
      (Or maybe use [leptos-markdown](https://github.com/rambip/leptos-markdown)? It supports [custom
      components](https://github.com/rambip/leptos-markdown/blob/main/examples/custom_component/src/main.rs))
      (Also note the same author made [dioxus-markdown](https://github.com/rambip/dioxus-markdown))
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
