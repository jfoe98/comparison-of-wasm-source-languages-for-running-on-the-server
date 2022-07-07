FROM rust

WORKDIR /usr/src/metrics-server
COPY . .

RUN cargo install --path .

EXPOSE 3000

CMD ["metrics-server"]