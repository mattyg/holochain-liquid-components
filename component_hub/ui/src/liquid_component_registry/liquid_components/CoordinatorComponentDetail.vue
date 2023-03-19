<template>
  <div v-if="!loading">
    <div v-if="editing" style="display: flex; flex: 1;">
      <EditCoordinatorComponent
        :original-coordinator-component-hash="coordinatorComponentHash"
        :current-record="record!"
        @coordinator-component-updated="editing = false; fetchCoordinatorComponent();"
        @edit-canceled="editing = false"
      ></EditCoordinatorComponent>
    </div>
    <div v-else-if="record" style="display: flex; flex-direction: column">
      <div style="display: flex; flex-direction: row">
        <span style="flex: 1"></span>
      
        <mwc-icon-button style="margin-left: 8px" icon="edit" @click="editing = true"></mwc-icon-button>
        <mwc-icon-button style="margin-left: 8px" icon="delete" @click="deleteCoordinatorComponent()"></mwc-icon-button>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Name: </strong></span>
 	<span style="white-space: pre-line">{{  coordinatorComponent?.name }} </span>
      </div>

    </div>
    
    <span v-else>The requested coordinator component was not found.</span>
  </div>

  <div v-else style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <mwc-snackbar ref="delete-error" leading>
  </mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { CoordinatorComponent } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import EditCoordinatorComponent from './EditCoordinatorComponent.vue';

export default defineComponent({
  components: {
    EditCoordinatorComponent
  },
  props: {
    coordinatorComponentHash: {
      type: Object,
      required: true
    }
  },
  data(): { record: Record | undefined; loading: boolean; editing: boolean; } {
    return {
      record: undefined,
      loading: true,
      editing: false,
    }
  },
  computed: {
    coordinatorComponent() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as CoordinatorComponent;
    }
  },
  async mounted() {
    if (this.coordinatorComponentHash === undefined) {
      throw new Error(`The coordinatorComponentHash input is required for the CoordinatorComponentDetail element`);
    }

    await this.fetchCoordinatorComponent();
  },
  methods: {
    async fetchCoordinatorComponent() {
      this.loading = true;
      this.record = undefined;

      this.record = await this.client.callZome({
        cap_secret: null,
        role_name: 'liquid_component_registry',
        zome_name: 'liquid_components',
        fn_name: 'get_coordinator_component',
        payload: this.coordinatorComponentHash,
      });

      this.loading = false;
    },
    async deleteCoordinatorComponent() {
      try {
        await this.client.callZome({
          cap_secret: null,
          role_name: 'liquid_component_registry',
          zome_name: 'liquid_components',
          fn_name: 'delete_coordinator_component',
          payload: this.coordinatorComponentHash,
        });
        this.$emit('coordinator-component-deleted', this.coordinatorComponentHash);
        this.fetchCoordinatorComponent();
      } catch (e: any) {
        const errorSnackbar = this.$refs['delete-error'] as Snackbar;
        errorSnackbar.labelText = `Error deleting the coordinator component: ${e.data.data}`;
        errorSnackbar.show();
      }
    }
  },
  emits: ['coordinator-component-deleted'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
})
</script>
