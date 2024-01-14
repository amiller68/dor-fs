#!/bin/bash

# Note: soemtimes my infura endpoint acts up and I have to use a curl request to upload files to IPFS
# I have to investigate why this is happening, but in the meantime, this script is a workaround

# Replace this with your actual command that outputs the required information
output=$(krondor-org device set sepolia && krondor-org device show)

# Extract the IPFS API base URL
api_base=$(echo "$output" | grep -o 'ipfs_remote: api_url: [^,]*' | cut -d' ' -f3)

credentials="${api_base#https://}"
credentials="${credentials%@*}"

# Separate username and password
uname="${credentials%%:*}"
pword="${credentials#*:}"

find sepolia -name '*' -type f -exec curl -X POST -F "file=@{}" -u "$uname:$pword" "$api_base/api/v0/add?cid-version=1&hash=blake3" \;cd 