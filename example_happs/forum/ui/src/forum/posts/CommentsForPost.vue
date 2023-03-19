
<template>
  <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
  
  <div v-else style="display: flex; flex-direction: column">
    <span v-if="error">Error fetching the comments: {{error.data.data}}.</span>
    <div v-else-if="hashes && hashes.length > 0" style="margin-bottom: 8px">
      <CommentDetail 
        v-for="hash in hashes" 
        :comment-hash="hash" 
      >
      </CommentDetail>
    </div>
    <span v-else>No comments found for this post.</span>
  </div>

</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import '@material/mwc-circular-progress';
import CommentDetail from './CommentDetail.vue';

export default defineComponent({
  components: {
    CommentDetail
  },
  props: {
    postHashHash: {
      type: Object,
      required: true
    }
  },
  data(): { hashes: Array<ActionHash> | undefined; loading: boolean; error: any } {
    return {
      hashes: undefined,
      loading: true,
      error: undefined
    }
  },
  async mounted() {
    try {
      this.hashes = await this.client.callZome({
        cap_secret: null,
        role_name: '',
        zome_name: 'posts',
        fn_name: 'get_comments_for_post',
        payload: this.postHashHash,
      });
    } catch (e) {
      this.error = e;
    }
    this.loading = false;
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
