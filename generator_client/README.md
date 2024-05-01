# generator-client

Generator-Client is a robust server designed for efficient management of the Kalypso Generator within the Oyster enclave. Its key functionalities include seamless control over kalypso generator (start, stop, restart), simplified generation and updating of generator configuration files, and the ability to fetch public keys for generators.

## enclave setup

**Note:** The <a target="_blank" href="https://github.com/marlinprotocol/kalypso-enclave-setups/tree/master/generator">enclavesetup</a> repo contains all necessary components for executing the generator-client and generator within the Oyster enclave. To ensure proper functionality, please follow these steps:

1. **Generate Fresh Release Builds:**
   - Create new release builds for both the generator-client and generator.
   - Place these builds within the `enclavesetup` directory.

2. **Utilize build.sh for EIF file Generation and Enclave Execution:**
   - Employ the `build.sh` script to accomplish the following tasks:
     - Generate the EIF file, which is crucial for enclave operation.
     - Launch the generator enclave, enabling its functionality.


## generator-client environment file setup

environment file for running outside the enclave:

```
PORT=5000
SUPERVISORD_PATH=./supervisord
```

`Note : SUPERVISORD_PATH is the path to the supervisord executable`

For running inside enclave, pass this to supervisord:

```
environment=PORT=5000,SUPERVISORD_PATH=./app/supervisord
```


## Steps to start the generator
1. Register the generator
   ```
   yarn test ./test/generatorOperations/1_register.ts
   ```

2. Stake tokens
   ```
   yarn test ./test/generatorOperations/2_stake.ts
   ```

3. Join a marketplace
   ```
   yarn test ./test/generatorOperations/3_join_market_place.ts
   ```

4. Update the ECIES key with the help of KalypsoSDK
    ```
   yarn test ./test/generatorOperations/4_update_ecies_key.ts
    ```
5. Generate the config setup by making an HTTP call to the generator client:
    ```
   curl --location --request POST 'http://43.205.85.160:5000/api/generatorConfigSetup' \
   --header 'Content-Type: application/json' \
   --data-raw '{
       "generator_config": [
       {
         "address": "0x0469866e13cd7DF08f5482FBb127a72fF197365D",
         "data": "Some data",
         "supported_markets": [
           "1"
         ]
       }
     ],
   
     "runtime_config": {
       "ws_url": "wss://arb-sepolia.g.alchemy.com/v2/********************",
       "http_url": "https://arb-sepolia.g.alchemy.com/v2/*******************",
       "start_block":29108940,
       "private_key": "******************************",
       "chain_id": 421614,
       "payment_token": "0x01d84D33CC8636F83d2bb771e184cE57d8356863",
       "staking_token": "0xdb69299dDE4A00c99b885D9f8748B2AeD1Fe4Ed4",
       "attestation_verifier": "0x3aB3487269206d5f6a10725d4e477BaA3611adcA",
       "entity_registry": "0xBf6AfC0dB112e1e330Ea3fF4640Bac5fBA3e4B65",
       "proof_market_place": "0x81C80965f4E1b073858cc9D55d7D9A517C9fF258",
       "generator_registry": "0x2CcCb1ac0fa40922bc800619E09fc3bD821ea4F8",
       "markets":{
           "1":{
               "port":"6000",
               "ivs_url":"****************"
           }
       }
     }
   
   }'
    ```
6. Start the zkbob-generator by invoking the following API call
    ```
    curl --location --request POST 'http://43.205.85.160:5000/api/startProgram' \
    --header 'Content-Type: application/json' \
    --data-raw '{
        "program_name":"zkbob-generator"
    }'
    ```
7. Start the kalypso-listener by invoking the following API call
    ```
    curl --location --request POST 'http://43.205.85.160:5000/api/startProgram' \
    --header 'Content-Type: application/json' \
    --data-raw '{
        "program_name":"listener"
    }'
    ```


## APIs

## End-point: getProgramStatus
### Method: GET
>```
>{{endpoint}}/api/getProgramStatus?program_name=listener
>```

`Note` : program_name is the name of the program in supervisord.

⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: startProgram
### Method: POST
>```
>{{endpoint}}/api/startProgram
>```

### Body (**raw**)
`Note` : program_name is the name of the program in supervisord.
```json
{
  "program_name":"zkbob-generator"
}
```

⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: restartProgram
### Method: POST
>```
>{{endpoint}}/api/restartProgram
>```

### Body (**raw**)
`Note` : program_name is the name of the program in supervisord.
```json
{
  "program_name":"listener"
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: stopProgram
### Method: POST
>```
>{{endpoint}}/api/stopProgram
>```

### Body (**raw**)
`Note` : program_name is the name of the program in supervisord.
```json
{
  "program_name":"zkbob-generator"
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: fetchGeneratorPublicKeys
### Method: POST
>```
>{{endpoint}}/api/fetchGeneratorPublicKeys
>```



### Body (**raw**)

```json
{
    "generator_address":"0xb05e1dA573707223574443AC6DD1054A9e3A451F"
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: generatorConfigSetup
### Method: POST
>```
>{{endpoint}}/api/generatorConfigSetup
>```



### Body (**raw**)

```json
   {
       "generator_config": [
       {
         "address": "0x0469866e13cd7DF08f5482FBb127a72fF197365D",
         "data": "Some data",
         "supported_markets": [
           "1"
         ]
       }
     ],
   
     "runtime_config": {
       "ws_url": "wss://arb-sepolia.g.alchemy.com/v2/***************",
       "http_url": "https://arb-sepolia.g.alchemy.com/v2/*************",
       "start_block":29108940,
       "private_key": "*********************************",
       "chain_id": 421614,
       "payment_token": "0x01d84D33CC8636F83d2bb771e184cE57d8356863",
       "staking_token": "0xdb69299dDE4A00c99b885D9f8748B2AeD1Fe4Ed4",
       "attestation_verifier": "0x3aB3487269206d5f6a10725d4e477BaA3611adcA",
       "entity_registry": "0xBf6AfC0dB112e1e330Ea3fF4640Bac5fBA3e4B65",
       "proof_market_place": "0x81C80965f4E1b073858cc9D55d7D9A517C9fF258",
       "generator_registry": "0x2CcCb1ac0fa40922bc800619E09fc3bD821ea4F8",
       "markets":{
           "1":{
               "port":"6000",
               "ivs_url":"*************"
           }
       }
     }
   }
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: updateRuntimeConfig
### Method: PUT
>```
>{{endpoint}}/api/updateRuntimeConfig
>```



### Body (**raw**)

```json
{
      "ws_url": "wss://withered-fluent-road.nova-mainnet.quiknode.pro/1b70d9337ca08c879ab8043747ff9e47d6f68fb6/",
      "private_key": "{{private-key}}",
      "proof_market_place": "0xDcF952396f63f2fd6cd762A9582db3f2B50716E7",
      "generator_registry": "0xE31295fba524Bb012A13B56F885A5dbd77A6d647",
      "start_block": 414025,
      "chain_id": 2
    }
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: addNewGenerator
### Method: POST
>```
>{{endpoint}}/api/addNewGenerator
>```



### Body (**raw**)

```json
    {
      "address": "0x01f01074dc5454B15faBf1F1006864D0b71e3f1c",
      "data": "Zeke",
      "supported_markets": [
        "0x07b7d625c70be57115ab18fc435ed0253425671cb91bd6547b7defbc75f52082"
      ]
    }
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: removeGenerator
### Method: DELETE
>```
>{{endpoint}}/api/removeGenerator
>```



### Body (**raw**)

```json
{
    "address":"0x01f01074dc5454B15faBf1F1006864D0b71e3f1c"
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: updateGeneratorConfig
### Method: PUT
>```
>{{endpoint}}/api/updateGeneratorConfig
>```



### Body (**raw**)

```json
{
    "address": "0x01f01074dc5454B15faBf1F1006864D0b71e3f1c",
    "data": "Armin",
    "supported_markets": ["0x6c2ec35f8128c43e710a84adb6c7de8978238ab2d2e2b9790847dbab464b54f9"]
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: get attestation document
### Method: GET
>```
>http://65.1.228.145:1300
>```

⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: build attestation
### Method: GET

`Note:  The endpoint here is the <ip:port> of the generator attestation-utility running inside the enclave. This is not a part of the generator-client.`

>```
>{{endpoint}}/attestation
>```
### Body (**raw**)

```json
{}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: verify attestation
### Method: POST

`Note:  The endpoint here is the <ip:port> of the whitelisted attestation verifier running inside the enclave. The endpoint will change in future. This is not a part of the generator-client.`

>```
>{{endpoint}}/verify
>```
### Body (**raw**)

`Note : The JSON body below is the response from the build attestation API.`

```json
{
    "attestation_doc": "8444a1013822a0591116a9696.......f86a2cc6ad5e45446fac8b8f8a54de",
    "pcrs": [
        "b1bcb1509699eb7de047caacc6b21a.....828b83c7425f485a7e22a334",
        "bcdf05fefccaa8e55bf2c8d6dee9.....e6b29c37ee80b214a414b7607236edf26fcb78654e63f",
        "12b0ba3f4ea54e8b38989d766.....0b69f1af52628db22b1edfc46d083d6a01c5e3ec4c"
    ],
    "min_cpus": 2,
    "min_mem": 5067894784,
    "max_age": 300000000,
    "signature": "6e48bc58566ec7b89379d19ddd38be0f1c4......7ec3f10dee790c218a2e1adab5cf025ca70a",
    "secp_key": "0498466048f3d4fd86c1439b493295......723721e81b8db08cf533a72b0a95e3fad2ff662e5060d8ac4a544814e"
}
```

⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃
### End-point: benchmark
### Method: GET
>```
>{{endpoint}}/api/benchmark?market_id=1
>```

### Result 

```
{
    "message": "Proof generated in 7755ms",
    "data": null
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃
## End-point: sign address
### Method: POST
>```
>{{endpoint}}/api/signAddress
>```
### Body (**raw**)

```json
{
    "address":"0xBEf74FB32CCeB599EFdE2fa3B2285A9462D207d1"
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: sign attestation
### Method: POST
>```
>{{endpoint}}/api/signAttestation
>```
### Body (**raw**)

```json
{
    "attestation":"0x8444a1013822a059111.....01caa1a2200972735acf37eee6d18992e989a7",
    "address":"0xBEf74FB32CCeB599EFdE2fa3B2285A9462D207d1"
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃




## Steps to benchmark the generator 

1. Update the ECIES key with the help of KalypsoSDK
    ```
   yarn test ./test/generatorOperations/4_update_ecies_key.ts
    ```
2. Generate the config setup by making an HTTP call to the generator client:
    ```
   curl --location --request POST 'http://43.205.85.160:5000/api/generatorConfigSetup' \
   --header 'Content-Type: application/json' \
   --data-raw '{
       "generator_config": [
       {
         "address": "0x0469866e13cd7DF08f5482FBb127a72fF197365D",
         "data": "Some data",
         "supported_markets": [
           "1"
         ]
       }
     ],
   
     "runtime_config": {
       "ws_url": "wss://arb-sepolia.g.alchemy.com/v2/********************",
       "http_url": "https://arb-sepolia.g.alchemy.com/v2/*******************",
       "start_block":29108940,
       "private_key": "******************************",
       "chain_id": 421614,
       "payment_token": "0x01d84D33CC8636F83d2bb771e184cE57d8356863",
       "staking_token": "0xdb69299dDE4A00c99b885D9f8748B2AeD1Fe4Ed4",
       "attestation_verifier": "0x3aB3487269206d5f6a10725d4e477BaA3611adcA",
       "entity_registry": "0xBf6AfC0dB112e1e330Ea3fF4640Bac5fBA3e4B65",
       "proof_market_place": "0x81C80965f4E1b073858cc9D55d7D9A517C9fF258",
       "generator_registry": "0x2CcCb1ac0fa40922bc800619E09fc3bD821ea4F8",
       "markets":{
           "1":{
               "port":"6000",
               "ivs_url":"****************"
           }
       }
     }
   
   }'
    ```
3. Start the zkbob-generator by invoking the following API call
    ```
    curl --location --request POST 'http://43.205.85.160:5000/api/startProgram' \
    --header 'Content-Type: application/json' \
    --data-raw '{
        "program_name":"zkbob-generator"
    }'
    ```
4. Benchmark the generator 
    ```
    curl --location --request GET 'http://43.205.85.160:5000/api/benchmark?market_id=1'
    ```
