<template>
  <div>
    <b>Search ComponentHub</b>
    <input type="text" v-model="text" @change="debounce(searchComponentHub, 200, {maxWait: 600})"/>
    <div v-if="results?.length > 0">
      <div v-for="result in results" :key="result.signed_action.hashed.hash.buffer.toString()" @click="$emit('change', result)">
        key: {{ result.signed_action.hashed.hash.buffer.toString() }}
        {{ result }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, defineProps } from 'vue';
import type { PropType } from 'vue';
import type { AppAgentWebsocket, Record } from '@holochain/client';
import debounce from 'lodash.debounce';

const text = ref("");
const results = ref<Array<Record>>([]);

const props = defineProps({
  componentHubAppAgentClient: {
    type: Object as PropType<AppAgentWebsocket>,
    required: true,
  }
});

const searchComponentHub = async () => {
  const records: Record[] = await props.componentHubAppAgentClient.callZome({
    role_name: 'component_hub',
    zome_name: 'registry',
    fn_name: 'search_coordinator_components',
    payload: text.value 
  });
  
  results.value = records;
};
</script>