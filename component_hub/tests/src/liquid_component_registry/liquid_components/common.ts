import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleCoordinatorComponent(cell: CallableCell, partialCoordinatorComponent = {}) {
    return {
        ...{
	  name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialCoordinatorComponent
    };
}

export async function createCoordinatorComponent(cell: CallableCell, coordinatorComponent = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "liquid_components",
      fn_name: "create_coordinator_component",
      payload: coordinatorComponent || await sampleCoordinatorComponent(cell),
    });
}

