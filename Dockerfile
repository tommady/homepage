FROM --platform=$BUILDPLATFORM docker.io/busybox:latest

WORKDIR /static
COPY static .

EXPOSE 9898/tcp
ENTRYPOINT ["httpd", "-f", "-p", "0.0.0.0:9898", "-h", "/static"]
