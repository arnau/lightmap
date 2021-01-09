# Lightmap

`lightmap` is a tool to map a SQLite database using the [DOT language].

It aims to behave like [sqleton] but as a single binary.


## Usage

```
lightmap test.db > test.dot && dot -Tsvg test.dot
```


## Licence

Licensed under MIT (See [LICENCE](./LICENCE)).

[DOT language]: https://www.graphviz.org/doc/info/lang.html
[sqleton]: https://github.com/inukshuk/sqleton
