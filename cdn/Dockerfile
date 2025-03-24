FROM golang:alpine AS build
WORKDIR /usr/src/cdn/dev

#* Download
COPY cdn/go.mod cdn/go.sum ./
RUN go mod download

#* Build
COPY ./cdn .

RUN go build

#* Release

FROM alpine
WORKDIR /usr/src/cdn/prod

COPY --from=build /usr/src/cdn/dev/saes-cdn /usr/src/cdn/prod/

RUN apk add --no-cache curl
HEALTHCHECK CMD curl --fail http://localhost:3100 || exit 1

EXPOSE 3100
ENTRYPOINT ["./saes-cdn"]
