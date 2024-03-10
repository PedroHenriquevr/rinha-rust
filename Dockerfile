FROM rust:1.76

COPY target/release/rinha-rust .

CMD ["./rinha-rust"]