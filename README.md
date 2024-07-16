# Solana Program for Genius Protocol


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
In the folder `genius-contracts-solana`

1. Install node modules using `yarn`

2. Build program using anchor cli `anchor build`

3. Get program address using solana cli.

    `solana-keygen pubkey ./target/deploy/genius-keypair.json`
   
   You can get the pubkey of the program. e.g. `BE4G...5qhv`

4. Change program address in the code to `BE4G...5qhv`

   in `lib.rs`
   ```
   declare_id!("BE4G...5qhv");
   ```
   in `Anchor.toml`
   ```
   genius = "BE4G...5qhv"
   ```

5. Change provider settings in `Anchor.toml`
   ```
   cluster = "devnet"
   wallet = "./admin.json"
   ```

6. run `anchor build` again

7. deploy program using anchor cli `anchor deploy`

<br/>

## Usage

### Initialize project

Initialize global variables and vault accounts.

```js
   yarn script init
```
<br/>

### Change admin of the program

Change admin address in the global state

```js
   yarn script change-admin -n <NEW_ADMIN_ADDRESS>
```
<br/>

### Update threshold

Update threshold percent in the global state

```js
   yarn script update-threshold -t <THRESHOLD_PERCENT>
```

<br/>

### Add orchestrator

Add a new orchestrator to the program

```js
   yarn script add-orchestrator -o <ORCHESTRATOR_ADDRESS>
```

<br/>

### Remove orchestrator

Remove orchestrator from the program

```js
   yarn script remove-orchestrator -o <ORCHESTRATOR_ADDRESS>
```

<br/>

### Add bridge liquidity

Orchestrator can adds liquidity to the program

```js
   yarn script add-bridge-liquidity -a <USDC_AMOUNT>
```

<br/>

### Remove bridge liquidity

Orchestrator can remove liquidity to the program

```js
   yarn script remove-bridge-liquidity -a <USDC_AMOUNT>
```

<br/>

### Swap deposit

User can deposit any token to the program

```js
   yarn script swap-deposit -t <TOKEN_ADDRESS> -a <TOKEN_AMOUNT>
```

<br/>

### Swap withdraw

Orchestrator can withdraw to any token and transfter to a user

```js
   yarn script swap-withdraw -u <USER_ADDRESS> -t <TOKEN_ADDRESS> -a <TOKEN_AMOUNT>
```

<br/>

### Withdraw stable coin

Orchestrator can withdraw USDC and transfter to a user

```js
   yarn script swap-withdraw -u <USER_ADDRESS> -a <TOKEN_AMOUNT>
```