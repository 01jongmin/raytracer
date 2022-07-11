FROM rust:1.62 as builder

WORKDIR ./raytracer
COPY ./raytracer-renderer ./raytracer-renderer

RUN USER=root cargo new --bin raytracer-server
WORKDIR ./raytracer-server
COPY ./raytracer-server/Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD ./raytracer-server ./

RUN rm ./target/release/deps/raytracer_server*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata wget \
    && rm -rf /var/lib/apt/lists/* \

EXPOSE 8080

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /raytracer/raytracer-server/target/release/raytracer-server ${APP}/raytracer-server

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}


CMD ["./raytracer-server"]
