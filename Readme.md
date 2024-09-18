# Monolith Application using Spin

This might be most abusing use of spin, as it is designed for microservices, but I wanted to push boundaries of what can be done with it.

There are examples available with leptos and dioxuslabs where spin can handle whole full stack application. 

Here, I am using Tera to create sample monolith application.

And to dedicate this to experience developers and get the nostalgia feeling, I have added JQUERY and Bootstrap.

## How to run

In current directory.

```bash
spin watch
```
It will watch current directory and run the spin applications 

--------- 

## Spin.toml
`files = ["templates/**/*.html"]`
is important as it will tell which files needed to be included in wasm application.

## Application
- Register email is doing nothing but displaying email and password. If confirm password is not same as password, it will show error.
- Login email is taking email and password and create a session cookie.
- Product page will display all products if cookie is present.
