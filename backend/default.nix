with import <nixpkgs> {};


stdenv.mkDerivation rec {
  name = "Yeet";
  env = buildEnv { name = name; paths = buildInputs; };
  buildInputs = [
    postgresql # Db and Db lib for Diesel
    openssl # Lib for TLS integration needed for Diesel + Warp
    pkgconfig # resolve dependencies
  ];
  shellHook = ''
    export PGDATA='pgsql'
    # to set the password, run `psql` and enter `\password` and set it to the password below
    export DATABASE_URL='postgres://hzimmerman:password@localhost/bucketquestions'
    export TEST_DATABASE_ORIGIN='postgres://hzimmerman:password@localhost'
    export DROP_DATABASE_URL='postgres://hzimmerman:password@localhost/postgres'
    export TEST_TYPE='UNIT'

    pg_ctl init
    pg_ctl -l db.logfile start -o "-h localhost -i"

    alias docs='cargo rustdoc --bins --open -- --document-private-items'
  '';
}
