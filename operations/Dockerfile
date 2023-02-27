FROM rust:1.65 as builder

ARG GITHUB_TOKEN
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

RUN git config --global url."https://${GITHUB_TOKEN}@github.com/".insteadOf "https://github.com/"

RUN USER=root cargo new --bin ticketland-signdrop-dist
COPY . ./ticketland-signdrop-dist
WORKDIR ./ticketland-signdrop-dist
RUN cargo build --release

RUN git config --global --unset url."https://${GITHUB_TOKEN}@github.com/".insteadOf

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /ticketland-signdrop-dist/target/release/ticketland-signdrop ${APP}/ticketland-signdrop

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENV ENV=production
CMD ["./ticketland-signdrop"]
