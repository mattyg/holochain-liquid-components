<template>
  <div v-if="loading">
    Loading...
  </div>
  <component :is="webcomponent" v-else></component>
</template>

<script setup lang="ts">
import { onMounted, type PropType } from 'vue';
import { defineProps, ref } from 'vue';
import { encodeHashToBase64, type Record } from '@holochain/client';
import { decode } from "@msgpack/msgpack";

const webcomponent = ref();
const loading = ref(true);

const props = defineProps({
  componentRecord: {
    type: Object as PropType<Record>,
    require: true
  },
  context: {
    type: Object,
    required: false
  }
});

onMounted(() => {
  loadWebComponent();
});

const loadWebComponent = async () => {  
  loading.value = true;

  const componentBytes = decode((props.componentRecord?.entry as any).Present.entry) as Uint8Array;
  const url = "data:text/javascript;base64," + encodeHashToBase64(componentBytes);

  /* @vite-ignore */
  webcomponent.value = await import(
    /* @vite-ignore */
    url
  );

  loading.value = false;
}
</script>