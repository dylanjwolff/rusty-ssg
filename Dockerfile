FROM rust
COPY . /opt/rusty-ssg
WORKDIR /opt/rusty-ssg
RUN cargo build --release
