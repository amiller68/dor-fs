echo 'Deploying contract to anvil...'
# This uses a private key that comes with anvil -- NEVER PUT YOUR OWN IN VERSION CONTROL
# But this key is fine for testing against a local development environment
# Also not the constructor args -- this will initialize the contract to point to an empty root CID
ADDRESS=$(forge \
	create \
	--rpc-url http://localhost:8545 \
	--chain 31337 \
	--private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
	src/device/eth/RootCid.sol:RootCid \
	--constructor-args "[0x0100000000000000000000000000000000000000000000000000000000000000,0x0000000000000000000000000000000000000000000000000000000000000000]" |
	grep -o 'Deployed to: [0-9a-fA-Fx]\+' | sed 's/Deployed to: //')

cargo run -- device create \
	--alias dev \
	--eth-rpc http://localhost:8545 \
	--contract-address ${ADDRESS} \
	--eth-chain-id 31337 \
	--ipfs-url http://localhost:5001 \
	--ipfs-gateway-url http://localhost:8080

cargo run -- device set dev

echo 'APP_CONTRACT_ADDRESS='${ADDRESS} > web.dev
echo 'APP_CHAIN_ID=31337' >> web.dev
echo 'APP_IPFS_GATEWAY_URL=http://localhost:8080' >> web.dev

