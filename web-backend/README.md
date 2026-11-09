Run with cargo:

```bash
cargo run
```

The `opencv` library requires some system libraries. If running gives you errors from `opencv`, follow its instructions
to install the required dependencies.

### Deployment

I ran into some problems with deploying because of `opencv` dependency issues and had to add some workarounds.
`build.sh` will build correctly when run on my server.
