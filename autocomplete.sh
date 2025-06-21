IMAGE=nds-rust-dev

FULL_PATH_CMD="cargo metadata --format-version 1 | jq -r '.packages[] | select(.name == \"libnds_sys\") | .manifest_path' | sed 's|/Cargo.toml||'"

echo "Building Docker image: $IMAGE"
docker build -t $IMAGE .

echo "Removing any existing temporary container named nds-temp"
docker rm -f nds-temp || true

echo "Creating temporary container to run cargo check"
docker run -d --name nds-temp -v $(pwd):/work $IMAGE sleep infinity

echo "Running cargo check in the temporary container"
docker exec nds-temp cargo +nightly check

echo "Retrieving full path of libnds_sys in Docker container"
DOCKER_FULL_PATH=$(docker exec nds-temp sh -c "${FULL_PATH_CMD}")
echo "Full path in Docker: ${DOCKER_FULL_PATH}"

echo "Retrieving full path of libnds_sys in host system"
SYSTEM_FULL_PATH=$(sh -c "$FULL_PATH_CMD")
echo "Full path in host: ${SYSTEM_FULL_PATH}"

echo "Copying libnds_sys from Docker container to host system"
echo "Copying from ${DOCKER_FULL_PATH} to ${SYSTEM_FULL_PATH}"
docker cp nds-temp:${DOCKER_FULL_PATH} ${SYSTEM_FULL_PATH}
