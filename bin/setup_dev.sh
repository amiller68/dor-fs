echo 'Deploying contract to anvil...'
# This uses a private key that comes with anvil -- NEVER PUT YOUR OWN IN VERSION CONTROL
ADDRESS=$(forge \
	create \
	--rpc-url http://localhost:8545 \
	--chain 31337 \
	--private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
	src/root_cid/RootCid.sol:RootCid \
	--constructor-args "0x0100000000000000000000000000000000000000000000000000000000000000" |
	grep -o 'Deployed to: [0-9a-fA-Fx]\+' | sed 's/Deployed to: //')

echo "Address: ${ADDRESS}"

cargo run -- configure create eth --alias dev --rpc http://localhost:8545 --address ${ADDRESS} --chain-id 31337
cargo run -- configure create ipfs --alias dev --url http://localhost:5001
cargo run -- configure set eth --alias dev
cargo run -- configure set ipfs --alias dev

cargo run -- configure show
