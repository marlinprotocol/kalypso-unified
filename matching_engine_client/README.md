# kalypso-matching-engine-client

The Kalypso matching engine client is a server designed for managing the Kalypso matching engine within the Oyster enclave. It controls functionalities like start, stop, restart, generate configuration files, fetching input verifier keys and interactions with the matching engine.

## Kalypso Enclave Setup

[Kalypso Enclave Setup](https://github.com/marlinprotocol/kalypso-enclave-setups/tree/master/matching-engine) is a set of scripts that launches a required enclave on an AWS nitro-cli compatible instance.

## Environment variables

```
PORT=6000
SUPERVISORD_PATH=./supervisord
MATCHING_ENGINE_PATH=testserver
```

`Note : SUPERVISORD_PATH is the path to the supervisord executable`

For running inside enclave, pass this to supervisord:

```
environment=PORT=5000,SUPERVISORD_PATH=./app/supervisord,MATCHING_ENGINE_PATH=testserver
```

## APIs

`Note : api-key provided below is a sample.`

## End-point: generateApiKey
### Method: POST
>```
>http://localhost:6000/api/generateApiKey
>```

⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: getMatchingEngineStatus
### Method: GET
>```
>http://localhost:6000/api/getMatchingEngineStatus
>```
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{me-api-key}}|



⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: startMatchingEngine
### Method: POST
>```
>http://localhost:6000/api/startMatchingEngine
>```
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{me-api-key}}|



⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: restartMatchingEngine
### Method: POST
>```
>http://localhost:6000/api/restartMatchingEngine
>```
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{me-api-key}}|



⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: stopMatchingEngine
### Method: POST
>```
>http://localhost:6000/api/stopMatchingEngine
>```
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{me-api-key}}|



⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: getMatchingEngineAddress
### Method: GET
>```
>http://localhost:6000/api/getMatchingEnginePublicKeys
>```
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{me-api-key}}|



⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: matchingEngineConfigSetup
### Method: POST
>```
>http://localhost:6000/api/matchingEngineConfigSetup
>```
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{me-api-key}}|


### Body (**raw**)

```json
{
    "rpc_url": "https://sepolia-rollup.arbitrum.io/rpc",
    "chain_id":421614,
    "relayer_private_key": "*****************************************",
    "proof_market_place": "0x83D452dD497c4Fd01a8e5531F336D084663Df0B1",
    "generator_registry": "0x82CeA7f50819e488C8D5C6D5D142d8a5e0A7b056",
    "start_block": 554025,
    "payment_token": "0x746F190DDaa001D2E42D768FFE46afD1720Cc493",
    "platform_token": "0xA713CB10e34EE0B1dD07af2E965602C118CD7be7",
    "attestation_verifier": "0xa60E856846bF736D0519ea0d38d2837a554F7862",
    "entity_registry": "0x6977Fc08E821e479e9BA00b2d37Ba9b85FCC0985"
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: updateMatchingEngineConfig
### Method: PUT
>```
>http://localhost:6000/api/updateMatchingEngineConfig
>```
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{me-api-key}}|


### Body (**raw**)

```json
{
    "chain_id":421614,
    "start_block": 484025
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: signAttestation
### Method: POST
>```
>http://localhost:6000/api/signAttestation
>```
### Headers

|Content-Type|Value|
|---|---|
|api-key|{{me-api-key}}|


### Body (**raw**)

```json
{
    "attestation":"0x0...00",
    "address":"0x04...5D"
}
```
