#! /bin/sh

r=$(basename $(pwd))
gql="COPY ./gql/src ./gql/src\nCOPY ./gql/Cargo.toml ./gql/Cargo.toml"
has_gql=false
for filename in ./*; do
    if [ $filename == "./gql" ];
    then
      has_gql=true;
    fi
done

echo "
# builder
FROM rust:latest as builder

WORKDIR /app
COPY ./rust_byc_helper/proc ./rust_byc_helper/proc
COPY ./rust_byc_helper/src ./rust_byc_helper/src
COPY ./rust_byc_helper/Cargo.toml ./rust_byc_helper/Cargo.toml
$(if $has_gql
then
echo $gql;
fi)
COPY ./src ./src
COPY Cargo.toml .
RUN cargo build -r -q

# runner
FROM debian:11.7-slim
COPY --from=builder /app/target/release/$r .
$(if $has_gql
then
echo "
# needed for gql
RUN apt-get update && apt-get install -y ca-certificates
RUN update-ca-certificates"
fi)

ENV RUST_LOG=\"info, _=error\"
ENTRYPOINT [\"./$r\"]" > dockerfile

