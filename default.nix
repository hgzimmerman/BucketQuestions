with import <nixpkgs> {};


stdenv.mkDerivation rec {
  name = "Yeet";
  env = buildEnv { name = name; paths = buildInputs; };
  buildInputs = [
    postgresql # Db and Db lib for Diesel
    openssl # Lib for TLS integration needed for Diesel + Warp
    pkgconfig # resolve dependencies
    geckodriver # Web driver for testing
  ];
  shellHook = ''
    export PGDATA='pgsql'
    # to set the password, run `psql` and enter `\password` and set it to the password below
    export DATABASE_URL='postgres://hzimmerman:password@localhost/bucketquestions'
    export TEST_DATABASE_URL='postgres://hzimmerman:password@localhost/bucketquestions_test'
    export TEST_DATABASE_NAME='bucketquestions_test'
    export DROP_DATABASE_URL='postgres://hzimmerman:password@localhost/postgres'

    pg_ctl init
    pg_ctl -l db.logfile start -o "-h localhost -i"

    alias docs='cargo rustdoc --bins --open -- --document-private-items'
  '';
}
