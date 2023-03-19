<template>
  <div>
    <h1>I am a LiquidComponentBasin</h1>
    <component-renderer v-if="componentRecord" :component="componentRecord"></component-renderer>
    <component-hub-search v-else v-model="componentRecord" :componentHubAppAgentClient="componentHubAppAgentClient"></component-hub-search>
  </div>
</template>

<script setup lang="ts">
import { ref, defineProps } from 'vue';
import type { PropType } from 'vue';
import type { AppAgentWebsocket } from '@holochain/client';

const componentRecord = ref();

defineProps({
  componentHubAppAgentClient: {
    type: Object as PropType<AppAgentWebsocket>,
    required: true,
  }
});

</script>