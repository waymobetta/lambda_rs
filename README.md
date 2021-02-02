# lambda-rs

a basic project template for writing lambda functions with rust

steps:

1. clone repo
```zsh
git clone git@github.com:waymobetta/lambda-rs.git
```

2. [macOS] install linker
```zsh
brew install filosottile/musl-cross/musl-cross
```

3. add linker to rustup target
```zsh
rustup target add x86_64-unknown-linux-musl
```

4. create lambda function within AWS

5. run build script
```zsh
chmod +x /scripts/proc.zsh
./scripts/proc.zsh
```

credits/resources

_the below resources are awesome and deserve all of the credit, I just found them a little confusing for getting started so I borrowed pieces from their research and simplified it_

- https://github.com/awslabs/aws-lambda-rust-runtime
- https://github.com/softprops/serverless-aws-rust
- https://github.com/SilentByte/rust-lambda

MIT
