# kalypso-zkb-input-verifier-client

The Kalypso input verifier client is a server designed for managing the Kalypso input verification service within the Oyster enclave. It controls functionalities like start, stop, restart, generate configuration files, fetching input verifier keys, etc.

## Kalypso Enclave Setup

[Kalypso Enclave Setup](https://github.com/marlinprotocol/kalypso-enclave-setups/tree/master/input-verifier) is a set of scripts that launches a required enclave on an AWS nitro-cli compatible instance.

## Generate API key
``` cmd
curl --location --request POST 'http://43.205.177.43:5000/api/generateApiKey'
```

## Generate the config setup
``` cmd
curl --location --request POST 'http://43.205.177.43:5000/api/generateConfigSetup' \
--header 'api-key: 07551368-f10f-4fc2-83ba-3d1d9cb0dacb'
```

## Start the input verifier
``` cmd
curl --location --request POST 'http://43.205.177.43:5000/api/startInputVerifier' \
--header 'api-key: 07551368-f10f-4fc2-83ba-3d1d9cb0dacb'
```

## Get status of input verifier
``` cmd
curl --location --request GET 'http://43.205.177.43:5000/api/getInputVerifierStatus' \
--header 'api-key: 07551368-f10f-4fc2-83ba-3d1d9cb0dacb'
```

## Get the public keys 
``` cmd
curl --location --request POST 'http://43.205.177.43:5000/api/fetchInputVerifierPublicKeys' \
--header 'api-key: 07551368-f10f-4fc2-83ba-3d1d9cb0dacb'
```

## Stop the input verifier
``` cmd
curl --location --request POST 'http://43.205.177.43:5000/api/stopInputVerifier' \
--header 'api-key: 07551368-f10f-4fc2-83ba-3d1d9cb0dacb'
```

## Sign Address 
``` cmd
curl --location --request POST 'http://43.205.177.43:5000/api/signAddress' \
--header 'api-key: 07551368-f10f-4fc2-83ba-3d1d9cb0dacb' \
--header 'Content-Type: application/json' \
--data-raw '{"address": "0x7dc...91f"}'
```

## Get attestation bytes signed (using "Hello worlds" as example)
```cmd
curl --location --request POST 'http://43.205.177.43:5000/api/signAttestation' \
--header 'api-key: 07551368-f10f-4fc2-83ba-3d1d9cb0dacb' \
--header 'Content-Type: application/json' \
--data-raw '{"attestation":"0x0..",
"address":"0x0.."}'
```

## Api calls for the input verifier
> Note: Encrypt the secrets with the above generated ecies keys

### Test 
``` cmd
curl --location --request GET 'http://43.205.177.43:3030/welcome'
```

### Check inputs
``` cmd
curl --location --request POST 'http://43.205.177.43:3030/checkInput' \
--header 'Content-Type: application/json' \
--data-raw '{"public_inputs":"0095...44d4","encrypted_secret":"b35...d25","acl":"046...497"}'
```

### Check inputs with signature
``` cmd
curl --location --request POST 'http://43.205.177.43:3030/checkInputWithSignature' \
--header 'Content-Type: application/json' \
--data-raw '{"ask_id":1,"public_inputs":"009...4d4","encrypted_secret":"4f6...2d7","acl":"04d...c62"}'
```
