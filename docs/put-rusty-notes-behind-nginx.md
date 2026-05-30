# Putting rusty-notes properly behind nginx auth

rusty-notes has no built-in authentication. When you expose it on a network,
you want a real login page with cookies/sessions — not nginx's `basic_auth`,
which pops an ugly browser username/password dialog and has no logout, no
expiry, and no session model.

The clean way to do this is nginx's **`auth_request`** (a.k.a. forward /
subrequest auth). A *separate* auth app owns login, sessions, and the cookie;
nginx just asks that app "is this request allowed?" before serving any
`/notes/` page.

## Why a URL prefix is involved at all

Directly hitting rusty-notes on its own port (`127.0.0.1:7777`), `/` is the
only prefix that makes sense — nothing else listens there. The prefix only
earns its keep behind a reverse proxy that multiplexes many apps onto one
public host/port (usually 443), distinguished by path:

```
https://example.com/notes/  →  nginx  →  127.0.0.1:7777  (rusty-notes)
https://example.com/auth/   →  nginx  →  127.0.0.1:9000  (auth app)
```

So we run rusty-notes with `RUSTY_URL_PREFIX=/notes/` and serve it under
`/notes/`, with the auth app alongside it under `/auth/`.

## How `auth_request` works

Before serving each `/notes/` request, nginx fires an *internal* subrequest to
the auth app. The auth app reads the incoming cookie and answers with a status
code only:

- **200** → nginx proceeds and proxies the real request to rusty-notes
- **401 / 403** → nginx blocks it; we turn that into a redirect to the login page

rusty-notes never sees any of this. It only ever receives requests nginx has
already approved.

## nginx config

```nginx
server {
    listen 443 ssl;
    server_name example.com;

    # --- the auth app owns these, publicly reachable ---
    location /auth/ {
        proxy_pass http://127.0.0.1:9000;
        proxy_set_header Host       $host;
        proxy_set_header Cookie     $http_cookie;
    }

    # internal-only verify endpoint (not reachable from outside)
    location = /auth/verify {
        internal;
        proxy_pass http://127.0.0.1:9000/verify;
        proxy_pass_request_body off;          # body not needed to check a cookie
        proxy_set_header Content-Length "";
        proxy_set_header Cookie $http_cookie;  # forward the session cookie
    }

    # --- the protected notes app ---
    location /notes/ {
        auth_request /auth/verify;            # gate every request

        # if verify returns 401, send the browser to the login page
        error_page 401 = @redirect_login;

        proxy_pass http://127.0.0.1:7777/;    # rusty-notes, RUSTY_URL_PREFIX=/notes/
        proxy_set_header Host             $host;
        proxy_set_header X-Forwarded-For  $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Prefix /notes;
    }

    location @redirect_login {
        return 302 /auth/login?next=$request_uri;
    }
}
```

Run rusty-notes with the matching prefix:

```sh
export RUSTY_SERVER_ADDR=127.0.0.1:7777
export RUSTY_URL_PREFIX=/notes/
```

## What the auth app does

A tiny service on `:9000` (Flask / FastAPI / Go / anything) with three jobs:

1. **`GET /auth/login`** — serve your own styled HTML login form (no browser
   popup).
2. **`POST /auth/login`** — check credentials, create a server-side session,
   and `Set-Cookie: session=...; HttpOnly; Secure; SameSite=Lax`. Then redirect
   to `next`.
3. **`GET /verify`** — read the session cookie; return `200` if the session is
   valid, `401` otherwise. nginx calls this on every request.

Because it's a real cookie/session you get logout, expiry, "remember me", etc.
— none of basic auth's limitations.

### Minimal sketch (FastAPI)

```python
import secrets
from fastapi import FastAPI, Request, Response, Form
from fastapi.responses import HTMLResponse, RedirectResponse

app = FastAPI()
SESSIONS: set[str] = set()           # swap for Redis/db in production
USERS = {"alice": "correct-horse"}   # swap for a real user store

@app.get("/auth/login", response_class=HTMLResponse)
def login_form(next: str = "/notes/"):
    return f"""
    <form method="post" action="/auth/login?next={next}">
      <input name="user"> <input name="pw" type="password">
      <button>Sign in</button>
    </form>"""

@app.post("/auth/login")
def login(next: str, user: str = Form(), pw: str = Form()):
    if USERS.get(user) != pw:
        return RedirectResponse("/auth/login", status_code=302)
    token = secrets.token_urlsafe(32)
    SESSIONS.add(token)
    resp = RedirectResponse(next, status_code=302)
    resp.set_cookie("session", token, httponly=True, secure=True, samesite="lax")
    return resp

@app.get("/verify")
def verify(request: Request):
    token = request.cookies.get("session")
    if token in SESSIONS:
        return Response(status_code=200)
    return Response(status_code=401)
```

## Even less work: an off-the-shelf auth proxy

If you'd rather write zero auth code, drop in a service that already speaks the
`auth_request` protocol and point `/auth/verify` at it:

- **[oauth2-proxy](https://github.com/oauth2-proxy/oauth2-proxy)** — acts as the
  verify endpoint, delegates login to Google/GitHub/any OIDC, and manages the
  session cookie itself. The standard choice for "this app has no auth, put SSO
  in front of it."
- **[Authelia](https://www.authelia.com/)** / **[Authentik](https://goauthentik.io/)**
  — full self-hosted login portals with their own users, 2FA, and sessions;
  both document the nginx `auth_request` integration directly.

## One caveat: it's all-or-nothing

Auth at the proxy gates `/notes/` as a whole. Every authenticated user gets the
same access rusty-notes provides, because rusty-notes has no per-user model. The
proxy can answer *"is this a logged-in user?"* but not *"can this user edit that
note?"*. If you only need to keep the public out, this is exactly right. Per-user
permissions would have to live in the app itself, which it does not currently
support.
