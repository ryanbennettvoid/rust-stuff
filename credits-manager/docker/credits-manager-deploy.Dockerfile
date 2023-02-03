FROM golang:1.16.7-alpine3.14

ENV GIN_MODE=release

WORKDIR /root

COPY ./.env ./.env
COPY ./cmd ./cmd
COPY ./pkg ./pkg
COPY ./vendor ./vendor
COPY ./go.mod ./go.mod

RUN go build ./cmd/art-generator/main.go

ENTRYPOINT ["./main"]