echo 'Deploying contract to anvil...'
# This uses a private key that comes with anvil -- NEVER PUT YOUR OWN IN VERSION CONTROL
ADDRESS=$(forge \
	create \
	--rpc-url http://localhost:8545 \
	--chain 31337 \
	--private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
	src/device/eth/RootCid.sol:RootCid \
	--constructor-args "[0x0100000000000000000000000000000000000000000000000000000000000000,0x0000000000000000000000000000000000000000000000000000000000000000]" |
	grep -o 'Deployed to: [0-9a-fA-Fx]\+' | sed 's/Deployed to: //')

echo "Address: ${ADDRESS}"

cargo run -- device create \
	--alias dev \
	--eth-rpc http://localhost:8545 \
	--contract-address ${ADDRESS} \
	--eth-chain-id 31337 \
	--ipfs-url http://localhost:5001 \
	--ipfs-gateway-url http://localhost:8080

cargo run -- configure device set --alias dev
