# syntax=docker/dockerfile:1
ARG PROVER_IMAGE
FROM us-docker.pkg.dev/matterlabs-infra/matterlabs-docker/prover-gpu-fri:2.0-$PROVER_IMAGE as prover

FROM nvidia/cuda:12.0.0-runtime-ubuntu22.04 as app

# HACK copying to root is the only way to make Docker layer caching work for these files for some reason
COPY *.bin /

RUN apt-get update && apt-get install -y libpq5 ca-certificates openssl && rm -rf /var/lib/apt/lists/*

# copy finalization hints required for assembly generation
COPY --from=prover prover/vk_setup_data_generator_server_fri/data/ /prover/vk_setup_data_generator_server_fri/data/
COPY --from=prover /usr/bin/zksync_prover_fri /usr/bin/

ENTRYPOINT ["zksync_prover_fri"]
