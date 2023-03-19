<template>
  <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <div v-else style="display: flex; flex-direction: column">
    <span v-if="error">Error fetching the coordinator components: {{error.data.data}}.</span>
    <div v-else-if="hashes && hashes.length > 0" style="margin-bottom: 8px">
      <CoordinatorComponentDetail 
        v-for="hash in hashes" 
        :coordinator-component-hash="hash"
        @coordinator-component-deleted="fetchCoordinatorComponent()"
      >
      </CoordinatorComponentDetail>
    </div>
    <span v-else>No coordinator components found.</span>
  </div>

</template>

<script lang="ts">
import { defineComponent, inject, toRaw, ComputedRef } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, NewEntryAction, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import '@material/mwc-circular-progress';
import CoordinatorComponentDetail from './CoordinatorComponentDetail.vue';
import { LiquidComponentsSignal } from './types';

export default defineComponent({
  components: {
    CoordinatorComponentDetail
  },
  data(): { hashes: Array<ActionHash> | undefined; loading: boolean; error: any } {
    return {
      hashes: undefined,
      loading: true,
      error: undefined
    }
  },
  async mounted() {
    await this.fetchCoordinatorComponent();
    toRaw(this.client).on('signal', signal => {
      if (signal.zome_name !== 'liquid_components') return; 
      const payload = signal.payload as LiquidComponentsSignal;
      if (payload.type !== 'EntryCreated') return;
      if (payload.app_entry.type !== 'CoordinatorComponent') return;
      if (this.hashes) this.hashes.push(payload.action.hashed.hash);
    });
  },
  methods: {
    async fetchCoordinatorComponent() {
      try {
        const records: Array<Record> = await this.client.callZome({
          cap_secret: null,
          role_name: 'liquid_component_registry',
          zome_name: 'liquid_components',
          fn_name: 'get_all_coordinator_components',
          payload: null,
        });
        this.hashes = records.map(r => r.signed_action.hashed.hash);
      } catch (e) {
        this.error = e;
      }
      this.loading = false;
    }
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
