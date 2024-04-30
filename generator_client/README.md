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
GENERATOR_PATH=kalypso-generator
```

`Note : SUPERVISORD_PATH is the path to the supervisord executable`

For running inside enclave, pass this to supervisord:

```
environment=PORT=5000,SUPERVISORD_PATH=./app/supervisord,GENERATOR_PATH=kalypso-generator
```


## Setps to start the generator

<ol>
  <li>Start the process by generating the api key by making a request to the endpoint: /api/generateApiKey (The generator api-key can only be generated once, so make sure you save it somewhere safe).</li>
  <li>Generate the config file by making a request to the endpoint: /api/generatorConfigSetup. </li>
  <li>Perform the SDK calls to register generator for a market: register, joinMarketPlace, updatePubkey ( Note : the attestation document and the generator's public keys can be fetched using the following SDK functions : getAttestation and getGeneratorPublicKeys ).</li>
  <li>Start the generator by making a request to the endpoint: /api/startGenerator</li>
</ol>


## APIs

`Note : api-key provided below is a sample.`

## End-point: generateApiKey
### Method: POST
>```
>{{endpoint}}/api/generateApiKey
>```

⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: getGeneratorStatus
### Method: GET
>```
>{{endpoint}}/api/getGeneratorStatus
>```
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{api-key}}|



⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: startGenerator
### Method: POST
>```
>{{endpoint}}/api/startGenerator
>```
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{api-key}}|



⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: restartGenerator
### Method: POST
>```
>{{endpoint}}/api/restartGenerator
>```
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{api-key}}|



⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: stopGenerator
### Method: POST
>```
>{{endpoint}}/api/stopGenerator
>```
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{api-key}}|



⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: fetchGeneratorPublicKeys
### Method: POST
>```
>{{endpoint}}/api/fetchGeneratorPublicKeys
>```
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{api-key}}|


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
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{api-key}}|


### Body (**raw**)

```json
{
      "generator_config": [
    {
      "address": "0xb05e1dA573707223574443AC6DD1054A9e3A451F",
      "data": "Eren",
      "supported_markets": [
        "0x07b7d625c70be57115ab18fc435ed0253425671cb91bd6547b7defbc75f52082"
      ]
    }
  ],

  "runtime_config": {
    "ws_url": "wss://withered-fluent-road.nova-mainnet.quiknode.pro/1b70d9337ca08c879ab8043747ff9e47d6f68fb6/",
    "http_url": "https://arbitrum-nova.publicnode.com",
    "private_key": "{{private-key}}",
    "start_block": 27432365,
    "chain_id": 42170,
    "payment_token": "0x28B6670dE9BD7fA4e718bd2D0fDdd27C8639c9b6",
    "generator_registry": "0x5f23d4eC6607fb381759151B02795187815b0487",
    "attestation_verifier": "0x90201271eD789Ed395ae2F1Cf5b0DC7FD3053176",
    "entity_registry": "0x9d3b60F3527f3c07bC0Afd15Bc842D191fCD5c6D",
    "proof_market_place": "0x2F1A3149D2798cA9Dfd7445B3cb713B4CD416b7d",
    "transfer_verifier_wrapper": "0x158E4B78f26A38978BdA2fC6F5326711D3b7a9D6",
    "zkb_verifier_wrapper": "0x0f782039EBDaba3D6f260298DFce5A35129BC716",
    "priority_list": "0x980fC290Ae6FBfD16f21C7e7a493F490acE4064C",
    "input_and_proof_format": "0x2424b9471b4B37F30d67F8B6b654B6b4720cD0a8",
    "staking_token": "0x57044769E5aa95590E4f99C174D9b90e9D7300F8",
    "zkbob_market_id":1
  }

}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: updateRuntimeConfig
### Method: PUT
>```
>{{endpoint}}/api/updateRuntimeConfig
>```
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{api-key}}|


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
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{api-key}}|


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
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{api-key}}|


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
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{api-key}}|


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
### Method: POST

`Note:  The endpoint here is the <ip:port> of the generator attestation-utility running inside the enclave. This is not a part of the generator-client.`

>```
>{{endpoint}}/build/attestation
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
>{{endpoint}}/verify/attestation
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
