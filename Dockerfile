# syntax=docker/dockerfile-upstream:master-labs
# check=experimental=all

FROM --platform=$BUILDPLATFORM alpine:latest AS ghost
ARG TARGETPLATFORM
ARG BUILDPLATFORM
WORKDIR /

COPY ./target/x86_64-unknown-linux-musl/release/ghostty-animation ./x86
COPY ./target/aarch64-unknown-linux-musl/release/ghostty-animation ./arm

RUN <<EOF
  # Building for ${TARGETPLATFORM}
  # Explicit target args are required: https://github.com/rust-lang/rust/issues/78210#issuecomment-714776007
  case "$TARGETPLATFORM" in
    *"linux/amd64"*)
      mv ./x86 ghost
      ;;
    *"linux/arm64"*)
      mv ./arm ghost
      ;;
    *)
      echo "Unsupported target platform: $TARGETPLATFORM";
      exit 1
      ;;
  esac
EOF

FROM scratch as release

# copy ghost
COPY --from=ghost /ghost /ghost
USER 65534:65534

FROM release
ENTRYPOINT [ "/ghost" ]
