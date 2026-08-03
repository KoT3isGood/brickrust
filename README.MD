# Brickrust
This is a SDK and mod loader for Brick Rigs written in fairly unsafe rust.

## Building and installing

Since the project 
```sh
# dev builds
make install DIR="/path/to/common/Brick Rigs" dev=true

# release builds
make install DIR="/path/to/common/Brick Rigs"
```

For usage with Proton you will need to add `WINEDLLOVERRIDES="xinput1_3=n,b" %command%`.
For now we do not support native linux builds as the game does not support modding very well.

## Docs

You can build docs by running
```sh
make doc
```

