# Run this in the root of the repo
make clean
make build
docker build -t acl -f ./contrib/auth/Dockerfile .