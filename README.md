# todomd - manage your per-project TODO.md

Well this project doesn't work yet, but,

Eventually it should offer a GUI that implements [this spec
here](https://github.com/todomd/todo.md) (or a superset of it) to manage
tasklists in a `TODO.md` files. (is this the same spec as [this
one](https://github.com/todo-md/todo-md)? Looks like the difference is "##" vs
"###" for each task list, maybe I should support both and other kinds of
structures). The important part is that when editing a markdown file, todomd
should preserve whitespace as much as possible, to minize diffs. If the user
wants to format the Markdown file in a specific way, they can run a linter
themselves.

As such, see the TODO.md file for an example (it's the todo for this app)

### But why?

Well for large projects it's better to use a proper collaborative kanban board
(maybe on Github or somewhere else). But for small, personal side projects, it's
sometimes convenient to track your progress through a `TODO.md`, [like
this](https://betterprogramming.pub/every-project-should-have-a-todo-md-file-20703bb6fd5f)
(note, article not written by me).

(Also note [there is a vscode
extension](https://marketplace.visualstudio.com/items?itemName=coddx.coddx-alpha)
that works exactly like this project should - maybe test it? The reason I don't
use it is that currently I am using Zed. It would be nice if they could
interoperate, at least for markdown files that actually follow its narrow spec
rather than having a more varied structure)

## Running

There's not much to run right now. I use `just` and `overmind` because that way
it's easy to run commands in any directory (rather than only at the project top
level) and so I can run both frontend and backend in the same terminal,
respectively.

### Debug cli

To run the debug cli, use the following command (you will need `just`)

```sh
just debug
```

(defaults to displaying `TODO.md`), or

```sh
just debug somefile.md
```

(If you don't have `just` installed, just check out `Justfile` to copy the exact
command to run)

### Dev server

To run the development tauri app + trunk server, run this (you will need both
`just` and `overmind`)

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
