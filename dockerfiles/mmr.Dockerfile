FROM alpine:latest
COPY --from=shadow:latest /usr/local/bin/shadow /usr/local/bin/shadow
CMD ["shadow", "run", "-vm", "--no-api", "--no-fetch"]
