# serverless-lambda-wasmtime
AWS Lambda WasmTime runtime with Serverless

# Quickstart
```shell
# install serverless
npm install -g serverless@latest

cd demo-rust

# install serverless plugins
npm install

# deploy app
sls deploy

```


# What does this do?
Demo of using serverless to run wasm binaries with [wasmtime](https://wasmtime.dev/). This example uses rust but you can use any language that compiles to wasm with [wasi](https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-intro.md) bindings.

I think it's neat that you can have a portable executable that can easily be run on AWS.
