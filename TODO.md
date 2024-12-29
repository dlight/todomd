# markanban - markdown kanban board

Should implement the spec in https://github.com/todomd/todo.md

(Note: is this the same as https://github.com/todo-md/todo-md? Is the difference only "##" vs "###" for each task list?)

(Also note [there is a vscode extension](https://marketplace.visualstudio.com/items?itemName=coddx.coddx-alpha) that works exactly like this kanban
board should work)

Also:

This is     a test.



### First, basic functionality
- [x] Pretty print markdown without changing it; see if it loses information
      (unfortunately, markdown-ast will not preserve the original markdown: it will strip two or more empty lines, and will escape the [ ]s,
      among other things. maybe markedit will fare better? or better, pulldown-cmark-to-cmark)
- [ ] Try configuring pulldown-cmark to add the required extension (tasklist) and use events_to_ast rather than markdown_to_ast
- [ ] Add a new task
- [ ] Remove a task
- [ ] Change name of a task
- [ ] Check/uncheck some box
- [ ] Add/remove a board

### Second, web interface
- [ ] Leptos hello world
- [ ] Render markdown myself (so I can edit it easily without another lib)
- [ ] Add button to create lists
- [ ] Add button to create tasks
- [ ] Button to remove lists
- [ ] Button to remove tasks
- [ ] Add editbox to edit a task name
