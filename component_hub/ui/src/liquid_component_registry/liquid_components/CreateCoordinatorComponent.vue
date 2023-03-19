<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create Coordinator Component</span>
  
    <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Name" :value="name" @input="name = $event.target.value" required></mwc-textfield>
    </div>

  
    <mwc-button 
      raised
      label="Create Coordinator Component"
      :disabled="!isCoordinatorComponentValid"
      @click="createCoordinatorComponent"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { CoordinatorComponent } from './types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';

export default defineComponent({
  data(): {
    name: string;
  } {
    return { 
      name: '',
    }
  },
  computed: {
    isCoordinatorComponentValid() {
    return true && this.name !== '';
    },
  },
  mounted() {
  },
  methods: {
    async createCoordinatorComponent() {
      const coordinatorComponent: CoordinatorComponent = { 
        name: this.name!,
      };

      try {
        const record: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'liquid_component_registry',
          zome_name: 'liquid_components',
          fn_name: 'create_coordinator_component',
          payload: coordinatorComponent,
        });
        this.$emit('coordinator-component-created', record.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['create-error'] as Snackbar;
        errorSnackbar.labelText = `Error creating the coordinator component: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['coordinator-component-created'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
