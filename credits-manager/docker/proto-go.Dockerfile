FROM golang:1.16.7-alpine3.14

RUN apk update && \
    apk add --no-cache protoc>3.21.1-r0 git &&\
    go get github.com/golang/protobuf/protoc-gen-go &&\
    cp /go/bin/protoc-gen-go /usr/bin/ &&\
    go get google.golang.org/grpc/cmd/protoc-gen-go-grpc &&\
    cp /go/bin/protoc-gen-go-grpc /usr/bin/

WORKDIR /root