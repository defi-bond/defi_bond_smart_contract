# Build Program

Creates a keypair (program account) and shared object (binary) file in the `target/deploy/` folder.
```cmd
$ cd <the program directory>
$ cargo build-sbf
```

# Deploy Program

Deploys the program's binary to the cluster and returns the `program id`.
```cmd
$ solana program deploy target/deploy/stake_pool_lotto.so
```

# Debugging

Outputs msg! calls
```cmd
$ solana logs
```

View transaction logs
```cmd
$ solana confirm -v <TRANSACTION_HASH>
```