FROM clux/muslrust
WORKDIR /volume
# silly docker copy about directory
# sh -c 'cd .. && docker build --network host -t demo -f actixweb-sqlx-jwt/Dockerfile .'
COPY actixweb-sqlx-jwt/ /volume/
RUN ls -lah && RUSTFLAGS='-C target-feature=+crt-static' cargo build --release && ls -lah target/*

FROM alpine:latest  
RUN apk --no-cache add ca-certificates
WORKDIR /opt/
COPY --from=0 /volume/target/x86_64-unknown-linux-musl/release/actixweb-sqlx-jwt /usr/bin
RUN actixweb-sqlx-jwt -V
CMD actixweb-sqlx-jwt -v

