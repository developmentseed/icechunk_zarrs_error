### Icechunk Zarrs Error
Quick and dirty reproducer for issue accessing array created with zarr-python in
Icechunk with zarrs.


```
uv run create_icechunk.py
```

This prints the tmp store location.

```
cargo run -- {tmp store location}
```


