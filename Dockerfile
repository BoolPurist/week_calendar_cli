
FROM rust:latest as builder

RUN apt update && apt upgrade -y 
RUN apt install -y g++-mingw-w64-x86-64 

RUN rustup target add x86_64-pc-windows-gnu 
RUN rustup toolchain install stable-x86_64-pc-windows-gnu 

WORKDIR /app 

COPY . .

RUN cargo build --target x86_64-pc-windows-gnu

FROM ubuntu:22.04


RUN mkdir -p /app/release

RUN yes | unminimize
RUN apt-get update
RUN apt-get upgrade --yes

COPY --from=builder /app/target/x86_64-pc-windows-gnu/release/week_calendar.exe /app/week_calendar.exe

#WORKDIR /app

# RUN apt-get install --no-install-recommends --assume-yes wine

# CMD ["cargo", "test", "--lib", "--target", "x86_64-pc-windows-gnu"]

