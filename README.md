# SecureTrust API

This repository contains reference implementations of the SecureTrust API.

## What is SecureTrust?

SecureTrust (formerly eMod Bans) is a server module for CS2D that allows global synchronisation of banlists between participating servers.
It is decentralised and does not require a central server to operate - anyone can run their own SecureTrust node.

## API contract

The SecureTrust API is an HTTP API designed to work with CS2D's legacy HTTP client. It operates exclusively using GET requests and query parameters,
and returns all of its responses as Lua tables.

### General guidelines

Every SecureTrust API implementation must be able to communicate using the HTTP/1.0 protocol and insecure HTTP.

Every API endpoint returns a response in the following general format:

```typescript
| { status: "ok", result: any, meta?: string }
| { status: "error", error: string, meta?: string }
```

The `status` field indicates whether the request was successful or not. If it is `ok`, the `result` field will contain the response data.
If it is `error`, the `error` field will contain a human-readable error message.

The `meta` field may contain additional information about the request or response, and may be displayed to game server operators
running the binding in debug mode to help diagnose issues.

If a mutation operation (such as `/add` or `/remove`) is for some reason a no-op, doing nothing but not actually failing,
then `status` must be `ok` and `result` must be `nil`.

Redirects are not supported with the exception of nginx's default redirects to trailing-slash versions of the same path for standard
PHP deployments.

### `GET /info`

This endpoint must return information about a node. It must always be public.

It may accept a `p` query parameter to return features unavailable without a valid password.

#### Example requests

```
$ curl --http1.0 --location 'http://127.0.0.1/info'
$ curl --http1.0 --location 'http://127.0.0.1/info?p=testpass'
```

#### Example responses

```lua
{
    status = "ok",
    result = {
        info = "This is a SecureTrust node operated by example.org.",
        contact = "you@example.org",
        features = { "list", "add", "remove" },
    }
}
```

### `GET /list`

This endpoint must return a list of ban records. It should generally be public.

It may accept a `p` query parameter for password authentication.

#### Example requests

```
$ curl --http1.0 --location 'http://127.0.0.1/list'
$ curl --http1.0 --location 'http://127.0.0.1/list?p=testpass'
```

#### Example responses

```lua
{
    status = "ok",
    result = {
        { target = "127.0.0.1", reason = "Test", time = -1 },
        { target = "7749", reason = "", time = -1 },
        { target = "127.0.1.*", reason = "", time = -1 },
    }
}
```

### `GET /add`

This endpoint must add a target to the node's list of ban records. It should generally be private.

It must accept a `target` query parameter to specify the ban target, which may be an IP or IP wildcard mask, a USGN ID, or a Steam ID (steamid64).

It may additionally accept a `reason` query parameter to specify the ban reason, which will be displayed to the user when they attempt to join a server.

It may additionally accept a `time` query parameter to specify the timestamp (in seconds, as returned by Lua's `os.time()`) after which the ban will no longer be valid.

It may accept a `p` query parameter for password authentication.

#### Example requests

```
$ curl --http1.0 --location 'http://127.0.0.1/add?target=127.0.0.1&reason=Speedhacking&time=1688666899&p=testpass'
```

#### Example responses

```lua
{
    status = "ok",
    result = "127.0.0.1",
}
```

### `GET /remove`

This endpoint must remove a target from the node's list of ban records. It should generally be private.

It must accept a `target` query parameter to specify the ban target, which may be an IP or IP wildcard mask, a USGN ID, or a Steam ID (steamid64).

It may accept a `p` query parameter for password authentication.

#### Example requests

```
$ curl --http1.0 --location 'http://127.0.0.1/remove?target=127.0.0.1&p=testpass'
```

#### Example responses

**Success**

```lua
{
    status = "ok",
    bans = {
        { target = "127.0.0.1", reason = "Test", time = -1 },
        { target = "7749", reason = "", time = -1 },
        { target = "127.0.1.*", reason = "", time = -1 },
    }
}
```

**Not found**

```lua
{
    status = "ok",
    result = false,
    meta = "Not found",
}
```
