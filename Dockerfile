FROM rust:1.88

WORKDIR /usr/src/chess_engine

COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

EXPOSE 6969

RUN cargo install --features service --path . 

CMD ["athena-chess", "6969"]

