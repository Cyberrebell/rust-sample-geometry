FROM alpine:latest AS base
WORKDIR /run
RUN apk add --no-cache \
 libgcc


FROM base AS build
RUN apk add --no-cache \
 rust \
 cargo
COPY . /run
RUN cargo build --release


FROM base AS app
CMD ./app
COPY --from=build /run/target/release/app /run