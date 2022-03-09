FROM --platform=$BUILDPLATFORM docker.io/busybox:latest

RUN mkdir static
COPY static static

EXPOSE 9898/tcp
RUN echo "E404:index.html" > /etc/httpd.conf
ENTRYPOINT ["httpd", "-f", "-p", "0.0.0.0:9898", "-h", "/static", "-c", "/etc/httpd.conf"]
