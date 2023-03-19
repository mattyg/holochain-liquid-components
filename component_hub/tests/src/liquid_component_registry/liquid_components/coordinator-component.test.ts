import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeDnaHash, fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createCoordinatorComponent, sampleCoordinatorComponent } from './common.js';

test('create CoordinatorComponent', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/liquid-components.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a CoordinatorComponent
    const record: Record = await createCoordinatorComponent(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read CoordinatorComponent', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/liquid-components.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const sample = await sampleCoordinatorComponent(alice.cells[0]);

    // Alice creates a CoordinatorComponent
    const record: Record = await createCoordinatorComponent(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created CoordinatorComponent
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "liquid_components",
      fn_name: "get_coordinator_component",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);
  });
});

test('create and update CoordinatorComponent', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/liquid-components.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a CoordinatorComponent
    const record: Record = await createCoordinatorComponent(alice.cells[0]);
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the CoordinatorComponent
    let contentUpdate: any = await sampleCoordinatorComponent(alice.cells[0]);
    let updateInput = {
      original_coordinator_component_hash: originalActionHash,
      previous_coordinator_component_hash: originalActionHash,
      updated_coordinator_component: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "liquid_components",
      fn_name: "update_coordinator_component",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated CoordinatorComponent
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "liquid_components",
      fn_name: "get_coordinator_component",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

    // Alice updates the CoordinatorComponent again
    contentUpdate = await sampleCoordinatorComponent(alice.cells[0]);
    updateInput = { 
      original_coordinator_component_hash: originalActionHash,
      previous_coordinator_component_hash: updatedRecord.signed_action.hashed.hash,
      updated_coordinator_component: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "liquid_components",
      fn_name: "update_coordinator_component",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated CoordinatorComponent
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "liquid_components",
      fn_name: "get_coordinator_component",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);
  });
});

test('create and delete CoordinatorComponent', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/liquid-components.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a CoordinatorComponent
    const record: Record = await createCoordinatorComponent(alice.cells[0]);
    assert.ok(record);
        
    // Alice deletes the CoordinatorComponent
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "liquid_components",
      fn_name: "delete_coordinator_component",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await pause(1200);
        
    // Bob tries to get the deleted CoordinatorComponent
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "liquid_components",
      fn_name: "get_coordinator_component",
      payload: record.signed_action.hashed.hash,
    });
    assert.notOk(readDeletedOutput);
  });
});
