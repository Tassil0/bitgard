# bitgard
GTK Bitwarden desktop client written in Rust.

packages
```

```

install schema

```
mkdir -p $HOME/.local/share/glib-2.0/schemas
cp com.tassilobalbo.bitgard.gschema.xml $HOME/.local/share/glib-2.0/schemas/
glib-compile-schemas $HOME/.local/share/glib-2.0/schemas/
```