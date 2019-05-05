# Bucket Questions

Bucket Questions a game like Truth or Dare, except there are no dares and who you question is randomly assigned.

This project is a formalization of the rules of the game into a webapp.


--------
### Deployment

* Clone the repository to wherever its hosted
* Make sure docker and docker-compose are installed
* Navigate to the ``./docker` directory
* Put in GOOGLE_CLIENT_ID and GOOGLE_CLIENT_SECRET into the dockerfile.
  * Make sure that these never leak into the repository
* Run `docker-compose -f docker-compose.yml up -d`
* Wait while it builds the image, and then builds the backend and frontend for the app.
  * You can run `docker-compose -f docker-compose.yml logs` to look at intermediate log output to see how far the build is.
  * When you see in the logs some pretty-printed-JSON-esque messages for server configuration, the server is up and ready to serve files.

### Development
When developing on NixOS, just navigate to `./backend` and run `nix-shell`.
This should pull all dependencies in and start a postgres server.
It doesn't pull in Node.js currently, so you still have to get that yourself.

If you don't want to use nix, a docker solution similar to the one above exists called `docker-compose-dev.yml`.
After starting that, you want to attach to the `app` container.

In both cases, you will have to put your `GOOGLE_CLIENT_ID` and `GOOGLE_CLIENT_SECRET` into the shell environment, because you don't want to leak those into version control.
The fastest way to develop is to use `npm start` for the frontend, and `cargo run -- --development --secret "SOME_SECRET"`.
This allows the hot-reloading utilized by `npm start`, and proxies all the requests to the development server.


