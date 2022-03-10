FROM --platform=$TARGETPLATFORM docker.io/busybox:latest

COPY static static

EXPOSE 9898/tcp
ENTRYPOINT ["httpd", "-f", "-p", "0.0.0.0:9898", "-h", "/static"]
