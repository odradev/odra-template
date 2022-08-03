# {{project-name}}


## Usage
It's recommend to install `[cargo-odra]`(https://github.com/odradev/cargo-odra) first.

### Build

```
$ cargo odra build
```
To build a wasm file, you need to pass the -b parameter. The result files will be placed in `${project-root}/wasm` directory.

```
$ cargo odra build -b casper
```

### Test
To run test on your local machine, you can basically execute the command:

```
$ cargo odra test
```

To test actual wasm files against a backend, you need to specify the backend passing -b argument to `cargo-odra`.

```
$ cargo odra test -b casper
```

### Backends

To build or test against a specific backend, you need pass a -b parameter to `cargo-odra`.
A list of available backends can check [here](https://github.com/odradev/odra).