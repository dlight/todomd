# todomd - manage your per-project TODO.md

Well this project doesn't work yet, but,

Eventually it should implement [this spec
here](https://github.com/todomd/todo.md) or a superset of it. (Also: is this the
same spec as https://github.com/todo-md/todo-md? Looks like the difference is
"##" vs "###" for each task list)

As such, see the TODO.md file for an example (it's the todo for this app)

(Also note [there is a vscode
extension](https://marketplace.visualstudio.com/items?itemName=coddx.coddx-alpha)
that works exactly like this kanban board should work - maybe test it? It would
be nice if they could interoperate, at least for markdown files that actually
follow its narrow spec rather than having more structure)

## Running

### Debug cli

To run the debug cli, use the following command (you will need `just`)

```sh
just debug
```

(If you don't have `just` installed, just check out `Justfile` to copy the exact
command to run)

### Dev server

To run the development tauri app + trunk server, run this (you will need `just`
and `overmind`)

```sh
overmind start
```

(Alternatively you can run `just backend` in a terminal, and `just frontend` in
another)

### Cleanup

To clean up build files, run this (you will need `just`)

```sh
just clean
```
