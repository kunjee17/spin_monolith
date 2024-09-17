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

In new tab, go to tera_parser project and run

```bash
cargo watch -w ../templates -x run
```
It will watch the templates directory and create templates.rs file in src directory. 
It is collection of string based templates that can be used from tera. 

Once strings are created from the html files, tera will take over from there and everything that can be 
done with tera can be done here. All other features are basically age-old http. Like once user login it will session cookie. And it will be checked in products page. 