FROM rust:alpine AS build
WORKDIR /usr/src/api/dev

#* Dependencies

RUN apk add --no-cache build-base 
RUN apk add --no-cache pkgconfig 
RUN apk add --no-cache openssl-dev 
RUN apk add --no-cache openssl-libs-static

#* Build
COPY ./api/ .
COPY ./shared ../shared

RUN cargo build --release

#* Release

FROM alpine
WORKDIR /usr/src/api/prod

ARG ENV_MODE
ENV ENV_MODE=${ENV_MODE}

ARG COMMIT_HASH
ENV COMMIT_HASH=${COMMIT_HASH}

COPY --from=build /usr/src/api/dev/target/release/saes-api /usr/src/api/prod/

RUN apk add --no-cache curl
HEALTHCHECK CMD curl --fail http://localhost:3000 || exit 1

RUN touch .env
EXPOSE 3000
ENTRYPOINT ["./saes-api"]
