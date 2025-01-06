# syntax=docker/dockerfile-upstream:master-labs
# check=experimental=all

FROM messense/cargo-zigbuild AS ghost
ARG TARGETPLATFORM
ARG BUILDPLATFORM

WORKDIR /work
COPY . /work

RUN <<EOF
  # Building for ${TARGETPLATFORM}
  case "$TARGETPLATFORM" in
    *"linux/amd64"*)
      cargo build --release --target x86_64-unknown-linux-musl
      mv ./target/x86_64-unknown-linux-musl/release/ghostty-animation ghost
      ;;
    *"linux/arm64"*)
      cargo build --release --target aarch64-unknown-linux-musl
      mv ./target/aarch64-unknown-linux-musl/release/ghostty-animation ghost
      ;;
    *)
      echo "Unsupported target platform: $TARGETPLATFORM";
      exit 1
      ;;
  esac
EOF

FROM scratch AS release

# copy ghost
COPY --from=ghost /work/ghost /ghost
USER 65534:65534

FROM release
ENTRYPOINT [ "/ghost" ]
