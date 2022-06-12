FROM wishlist-build as build

LABEL stage=wishlist-build

COPY . .
RUN scripts/build.sh

FROM debian:buster-slim

WORKDIR /app
COPY --from=build /build/target/release/app .

CMD ["./app"]
