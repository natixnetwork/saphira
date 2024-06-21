# Prepare

- create a solana address as a payer
    ```
    solana-keygen new -o payer.json
    ```
- create a solana address as a owner of the program that is the only address who can set config and init accounts for program
    ```
    solana-keygen new -o auth.json
    ```
- create an account for the token and use that account for staking account.
    ```
    solana-keygen new -o staking.json
    solana config set -k ./staking.json
    spl-token create-account mint-address --fee-payer payer.json
    ```


set according values inside constants.js

set the right value for get_owner_id.rs (Auth), get_natix_token_mint.rs, get_natix_token_id.rs, get_stake_account_id.rs

# Build
```
cargo build-bpf
```

# Deploy
```
solana program deploy target/deploy/stake.so
```

you can specify --program id if you want to upgrade the contract.

```
solana program deploy target/deploy/stake.so --program-id XXXXXX
```

set the program id in constants.js


# ProgramAccount

- create a solana address
    ```
    solana-keygen new -o programAccount.json
    ```
- set the according value in constans.js
- edit createAccount.js and set the length to the right value.
- use createAccount.js to associate that account to program.


## How To Have New ProgramAccount

- run initPda.js to set the authority of the staking account to the pda (Only one time needed)
- run initRequest.js to initiate the programAcccount (Needed for every new programAccount)
- modify setConfig.js and set the right config and run setConfig.js to provision the programAccount
- modify changeInterest.js to set the right interest and run changeInterest.js to set the right interest
- run readProgramAccount.js to read the values and make sure everything is correct.
now you can use this programAccount.





