## Lita example

This repo provides an example of lita zk-vm, this includes:

-   An example program
-   A verifier
-   A webapp that uses the wasm integration

Refer to [lita official docs](https://lita.gitbook.io/lita-documentation) for more info.

### Requirements

**Valida toolchain supported version: 0.9.0-alpha**

Follow their docs [here](https://lita.gitbook.io/lita-documentation/quick-start/installation-and-system-requirements) to install the valida toolchain.

### Try it

Compile the program:

```shell
make compile_program
```

Run program and store stdout:

```shell
make run_program
```

> Note:
> The input for the program is taken from: `test_data/stdin`. You can change it to run different scenarios.

Prove program:

```shell
make prove_program
```

Verify program:

```shell
make verify_program
```

Run web app and prove and verify the program in the browser:

```shell
make run_web_app
```
