<h1 align="center"> Krondor.Org </h1>

This is a combined CMS and personal Web3 blog that I wrote for myself in Rust. This project has been a long process in failure, learning, and experimentation with technologies like the EVM, Ipfs, Rust, and WASM. This is still a prototype, and I'm not sure if I want to continue down this path, but I welcome pull requests and feedback.

## Requirements
- Rust
- Yarn
- Trunk
- Foundry
- Ipfs
- Anvil

## The Stack

There are four main components to this project:

- An Ipfs node that stores the content of the site. This node should be writable by the owner and readable by anyone through a public gateway.
- A RootCid contract that points to the latest version of the site's content, as specified within a `manifest` file. This `manifest` is hashed, pushed to Ipfs, and used to link to site data and metadata.
- A CLI that is able to pull, update, and push new content to the Ipfs node and update the RootCid contract.
- A Web App that is able to read the Cid from the RootCid contract and display the current content of the site as specified by the `manifest` file.

## Setup

Be sure to clone this repo with the `--recursive` flag to pull in the submodules.

```bash
git clone --recurse-submodules
```

A valid ABI is included in version control, but if you need to compile the solidity contracts yourself, you can do so with the following:

```bash
forge build
```

## Development

If you just want to see a demo of the site with some sample data, you can run the following:

```bash
# Start a local Ipfs one terminal
ipfs daemon
# Start a local Anvil node in another terminal
anvil
# Push some initial content to your local nodes
./bin/reset_dev_env.sh
```

At which point you should be able to serve the site with trunk using:

```bash
yarn dev
```

The site should be available at `http://localhost:3000`

## Deployment

To deploy the site you'll need to have access to the following:

- An Ipfs API endpoint that you can write to, and want to host your site data on. I use Infura for this.
- Access to an EVM RPC endpoint that you can write to and publish to version control. I use Infura for this as well.
- A wallet with enough Eth to pay for gas fees. Pro Tip: I just use Sepolia for this because gas fees are crazy high! This is a problem that I think I'll eventually solve by just deploying and trusting my own infrastucture (think more of a Web2 approach).

### Contract Deployment

You can deploy the RootCid contract with forge. This command will create the new contract, initialize it with a Default Cid, and give you the address of the new contract.

```bash
forge \
	create \
	--rpc-url <YOUR_RPC_URL> \
	--chain <YOUR_CHAIN_ID> \
	--private-key <YOUR_PRIVATE_KEY> \
	src/device/eth/RootCid.sol:RootCid \
	--constructor-args "[0x0100000000000000000000000000000000000000000000000000000000000000,0x0000000000000000000000000000000000000000000000000000000000000000]"
```

### CLI Device Setup

Once you have a contract deployed, you can create a new CLI device that will be able to interact with it. This command will create a new device on your local machine with the needed configuration to interact with the contract, as well as push and pull content from Ipfs.

```bash
cargo run -- device create \
	--alias <DEVICE_NAME> \
	--eth-rpc <YOUR_RPC_URL> \
	--contract-address <CONTRACT_ADDRESS>
	--eth-chain-id <YOUR_CHAIN_ID> \
	--ipfs-url <YOUR_IPFS_API_URL> \
	--ipfs-gateway-url <YOUR_IPFS_GATEWAY_URL> \
```

Note: you're Ipfs API url should include any necessary authentication information, such as a username and password, if you're using a service like Infura.

At this point you should populate the `web.config` file with:
- your contract address
- your chain id
- your public eth rpc url
- your Ipfs gateway url

So that the web app can read the contract and display the latest content.

### CLI Usage

You must have a local Ipfs node running to use the CLI. You can start one with:

```bash
ipfs daemon
```

After you create a device, you should be able to use it to push and pull content from Ipfs and update the RootCid contract. You can see the available commands with:

```bash
cargo run -- --help
```

But here's a quick overview of the commands you'll need to use:

```bash
# Initialize a new space to pull and stage changes from in the current directory
cargo run -- init
# Pull the latest content from Ipfs and update the local staging area
cargo run -- pull
# Stage changes from the current directory against the local staging area
cargo run -- stage
# You can also tag files with metadata that will be stored in the manifest
# Here are example tags that are used in the development environment setup
# Creates a new piece of 'audio' content
cargo run -- tag --name audio --path freak-mic-test.mp3 --value '{"title": "Freak on a Leash (Sample)", "project": "mic_test"}'
# Creates a new piece of 'writing' content
cargo run -- tag --name writing --path hello_world.md --value '{"title": "Hello World", "description": "A lil hello!", "genre": "blog"}'
# Creates a new piece of 'visual' content
cargo run -- tag --name visual  --path petting_turtles.jpg --value '{"title": "Draw me, Naked, Petting the Turtles", "location": "New York", "medium": "blue ink on lined paper"}'
# Push the staged changes to Ipfs and update the RootCid contract
cargo run -- --dir playground --admin-key <YOUR_PRIVATE_KEY> push
```

You can check the status of your device with:

```bash
cargo run -- health
```

### Web App Usage

If you want to see the results of your CLI changes, you can run the following:

```bash
yarn start
```

The site should be available at `http://localhost:3000`

