# A POC auth service for DDRaceNetwork servers

Work in progress

## Endpoints

| METHOD | Endpoint | Data | What it does |
| --- | --- | --- | -- |
| GET | `/version` | Auth server version, e.g 1.0.0 | Tell the backend version |
| POST | `/account/register` | A Json object "RegisterPayload" (check below for how its defined) | Register a new account |
| POST | `/account/mapping` | A json object "AccoundIdRequest" (check below for how its defined) | Given a public key, get its mapped uuid |
| POST | `/account/verify` | A json object "VerifyUserPayload" (check below for how its defined) | Verify a public key is valid and return the account id |

### RegisterPayload
```json
{
    "public_key": "base64 encoded public key byte data",
    "email": "email@email.com",
    "email_signature": "base64 encoded signature of the email using the private key for the given public key",
}
```

### AccoundIdRequest
```json
{
    "public_key": "base64 encoded public key byte data",
}
```

### VerifyUserPayload
```json
{
    "public_key": "base64 encoded public key byte data",
    "message": "something here",
    "message_signature": "base64 encoded signature of the message using the private key for the given public key",
}
```


## TODO

- [ ] Add a packet on ddnet to let the client send the public key to the server to verify its identity.
- [x] Add a way to let the client register sending an email + public_key + the signature of the email using the private key.

https://www.openssl.org/docs/man1.1.1/man7/Ed25519.html
