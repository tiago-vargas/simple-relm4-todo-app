# simple-relm4-todo-app

Really simple To-Do app I'm making to learn GTK, Rust and Relm4.

![Screenshot from 2023-11-18 14-05-45](https://github.com/tiago-vargas/simple-relm4-todo-app/assets/78927143/ea74de91-495b-4a8a-a469-2032d26ac280)


## Installing

The `./` directory is the project folder you just cloned or extracted.

Create the directory (if non existant) for the schema:

``` shell
mkdir -p ~/.local/share/glib-2.0/schemas/
```

Copy the schema there:

``` shell
cp ./data/com.github.tiago_vargas.simple_relm4_todo.gschema.xml ~/.local/share/glib-2.0/schemas/
```

Compile the schema there:

``` shell
glib-compile-schemas ~/.local/share/glib-2.0/schemas/
```

Build the app:

``` shell
cargo build --release
```

The app is outputed to `./target/release/`.
Just run it from there.

``` shell
./target/release/simple-relm4-todo
```


## Uninstalling

The app is self-contained, so you could just delete it.

If you want to leave things as if you never installed it

- Delete the schema:
	``` shell
	~/.local/share/glib-2.0/com.github.tiago_vargas.simple_relm4_todo.gschema.xml
	```

The app saves your tasks to `~/.local/share/`.
Just delete the folder `com.github.tiago_vargas.simple_relm4_todo/` there.
