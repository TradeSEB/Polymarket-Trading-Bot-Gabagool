# Spl token presale smart contract


## Install Dependencies

- Install `node` and `yarn`

- Install `rust`, `solana` and `anchor`

    https://www.anchor-lang.com/docs/installation

- Prepare solana wallet for the test

    ```
    solana-keygen new -o admin.json
    ```

    Airdrop/Transfer SOL to this test wallet.

<br/>

## How to deploy this program?

First of all, you have to clone this repo to your PC.
In the folder `token-presale`

1. Install node modules using `yarn`

2. Build program using anchor cli `anchor build`

3. Get program address using solana cli.

    `solana-keygen pubkey ./target/deploy/presale-keypair.json`
   
   You can get the pubkey of the program. e.g. `BE4G...5qhv`

4. Change program address in the code to `BE4G...5qhv`

   in `lib.rs`
   ```
   declare_id!("BE4G...5qhv");
   ```
   in `Anchor.toml`
   ```
   presale = "BE4G...5qhv"
   ```

5. Change provider settings in `Anchor.toml`
   ```
   cluster = "localnet"
   wallet = "./admin.json"
   ```

6. run `anchor build` again

7. deploy program using anchor cli `anchor deploy`

<br/>

## Usage

### Test project

./tests/presal.ts is test script for each instruction


run `anchor test` to check test the smart contract
