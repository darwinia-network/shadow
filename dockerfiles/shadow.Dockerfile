FROM alpine:latest
COPY --from=shadow:latest /usr/local/bin/shadow /usr/local/bin/shadow
EXPOSE 3000
CMD ["shadow", "run", "-v"]
