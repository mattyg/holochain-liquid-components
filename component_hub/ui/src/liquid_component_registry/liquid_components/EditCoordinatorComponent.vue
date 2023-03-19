<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Edit Coordinator Component</span>
      <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Name" :value="name" @input="name = $event.target.value" required></mwc-textfield>
      </div>



    <div style="display: flex; flex-direction: row">
      <mwc-button
        outlined
        label="Cancel"
        @click="$emit('edit-canceled')"
        style="flex: 1; margin-right: 16px;"
      ></mwc-button>
      <mwc-button 
        raised
        label="Save"
        :disabled="!isCoordinatorComponentValid"
        @click="updateCoordinatorComponent"
        style="flex: 1;"
      ></mwc-button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { CoordinatorComponent } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';

export default defineComponent({
  data(): {
    name: string;
  } {
    const currentCoordinatorComponent = decode((this.currentRecord.entry as any).Present.entry) as CoordinatorComponent;
    return { 
      name: currentCoordinatorComponent.name,
    }
  },
  props: {
    originalCoordinatorComponentHash: {
      type: null,
      required: true,
    },
    currentRecord: {
      type: Object,
      required: true
    }
  },
  computed: {
    currentCoordinatorComponent() {
      return decode((this.currentRecord.entry as any).Present.entry) as CoordinatorComponent;
    },
    isCoordinatorComponentValid() {
      return true && this.name !== '';
    },
  },
  mounted() {
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord input is required for the EditCoordinatorComponent element`);
    }
    if (this.originalCoordinatorComponentHash === undefined) {
      throw new Error(`The originalCoordinatorComponentHash input is required for the EditCoordinatorComponent element`);
    }
  },
  methods: {
    async updateCoordinatorComponent() {

      const coordinatorComponent: CoordinatorComponent = { 
        name: this.name,
      };

      try {
        const updateRecord: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'liquid_component_registry',
          zome_name: 'liquid_components',
          fn_name: 'update_coordinator_component',
          payload: {
            original_coordinator_component_hash: this.originalCoordinatorComponentHash,
            previous_coordinator_component_hash: this.currentRecord.signed_action.hashed.hash,
            updated_coordinator_component: coordinatorComponent
          }
        });
        this.$emit('coordinator-component-updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['update-error'] as Snackbar;
        errorSnackbar.labelText = `Error updating the coordinator component: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['coordinator-component-updated', 'edit-canceled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
