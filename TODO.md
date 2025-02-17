### Ok uhm

I am not using this very consistently right now but that's ok, that's what's
this tool is for...

Stuff relegated to a distant future can be found in `BACKLOG.md`. When it's time
to do something *now*, it is moved here.

- [x] Use Leptos' view builder syntax to build some random dynamic content.
- [ ] Read markdown in Tauri and pass it through a command.
- [ ] Display markdown in `<pre>`
- [ ] Highlighting a single span of text (like, the first item, of the first list).
- [ ] Highlight multiple spans of text.

### TODO notes

(There is also `NOTES.md` but it's for things more long term)

At least initially, I will do it myself. [`leptos-markdown`'s onclick
example(https://github.com/rambip/leptos-markdown/blob/main/examples/onclick/src/main.rs#L32-L49)
just does this, and it looks easy enough to do manually. I just can't use `view!
{}` because it's not a static number of spans, but depends on the number of todo
items. So I will need to use the [view builder
syntax](https://book.leptos.dev/view/builder.html).

Eventually, when `leptos-markdown` becomes compatible with leptos 0.7, I might
use it.
