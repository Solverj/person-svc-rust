FROM debian:bullseye-slim
WORKDIR /app
ADD target/release/person-svc .
CMD ["app/person-svc"]