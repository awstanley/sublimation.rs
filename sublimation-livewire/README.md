# sublimation-livewire

sublimation to electron interface

## Notes

For Windows (due to [neon#357](https://github.com/neon-bindings/neon/issues/357)):

```bash
neon build --release
```

Also note that `neon build` can/will fail if run from this directory.  You will need to compile it by hand due to the workspace setting at the top of this repository.