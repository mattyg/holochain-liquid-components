# Liquid Components

Small units of composable UI+backend for your Holochain apps.

## Architectural Elements:
- ComponentHub: A hApp for publishing your LiquidComponents
- UI Basin Input: A "placeholder" component which can be filled by any LiquidComponent that meets it's constraints
  - Search ComponentHub for compatible components that meet the Basin's constraints
  - Select a LiquidComponent
    - A compatible Cell is installed to your conductor
    - The WebComponent is downloaded
    - The WebComponent is rendered and connected to the newly installed Cell
- UI Basin Renderer: Renders an already defined UI Basin's chosen WebComponent

## TODO
- [ ] Props for allowlist & denylist of ComponentHub hashes in the LiquidComponentBasin
- [ ] Props for allowlist & denylist of ComponentHub hashes in the ComponentHubSearch
- [ ] Add PR for DevHub for coordinator zome function `get_dnas_with_all_zomes(zomes: Vec<EntryHash>)`' to fetch a list of dnas that contain *all* the necessary zomes
    - [ ] Refactor to use an array of coordinator zomes on devhub as the core definition of  compatibility, instead of an array of DNAs on devhub. This will allow us to generate the list of compatible DNAs by querying `get_dnas_with_all_zomes`

## Environment Setup

> PREREQUISITE: set up the [holochain development environment](https://developer.holochain.org/docs/install/).

Enter the nix shell by running this in the root folder of the repository: 

```bash
nix-shell
npm install
```

**Run all the other instructions in this README from inside this nix-shell, otherwise they won't work**.

## Running 2 agents
 
```bash
npm start
```

This will create a network of 2 nodes connected to each other and their respective UIs.
It will also bring up the Holochain Playground for advanced introspection of the conductors.

## Running the backend tests

```bash
npm test
```

## Bootstrapping a network

Create a custom network of nodes connected to each other and their respective UIs with:

```bash
AGENTS=3 npm run network
```

Substitute the "3" for the number of nodes that you want to bootstrap in your network.
This will also bring up the Holochain Playground for advanced introspection of the conductors.

## Packaging

To package the web happ:
``` bash
npm run package
```

You'll have the `liquid-components.webhapp` in `workdir`. This is what you should distribute so that the Holochain Launcher can install it.
You will also have its subcomponent `liquid-components.happ` in the same folder`.

## Documentation

This repository is using these tools:
- [NPM Workspaces](https://docs.npmjs.com/cli/v7/using-npm/workspaces/): npm v7's built-in monorepo capabilities.
- [hc](https://github.com/holochain/holochain/tree/develop/crates/hc): Holochain CLI to easily manage Holochain development instances.
- [@holochain/tryorama](https://www.npmjs.com/package/@holochain/tryorama): test framework.
- [@holochain/client](https://www.npmjs.com/package/@holochain/client): client library to connect to Holochain from the UI.
- [@holochain-playground/cli](https://www.npmjs.com/package/@holochain-playground/cli): introspection tooling to understand what's going on in the Holochain nodes.
