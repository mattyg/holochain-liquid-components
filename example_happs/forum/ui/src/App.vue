<template>
  <div>
    <div v-if="loading">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>
    <div v-else>
      <main>
        <h1>Forum</h1>
      
        <div id="content">
          <h2>All Posts</h2>
          <AllPosts></AllPosts>
          <span style="margin-bottom: 16px"></span>
          <CreatePost></CreatePost>
        </div>

        <h1>LiquidComponentBasin</h1>
        <liquid-component-basin :componentHubAppAgentClient="componenthubClient"></liquid-component-basin>
      </main>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, computed } from 'vue';
import { AppAgentWebsocket, AppAgentClient } from '@holochain/client';
import '@material/mwc-circular-progress';
import AllPosts from './forum/posts/AllPosts.vue';
import CreatePost from './forum/posts/CreatePost.vue';

export default defineComponent({
  components: {
    // Add your subcomponents here
    AllPosts,
    CreatePost
  },
  data(): {
    client: AppAgentClient | undefined;
    componenthubClient: AppAgentClient | undefined;
    loading: boolean;
  } {
    return {
      client: undefined,
      componenthubClient: undefined,
      loading: true,
    };
  },
  async mounted() {
    // We pass '' as url because it will dynamically be replaced in launcher environments
    this.client = await AppAgentWebsocket.connect('', 'forum');
    this.componenthubClient = await AppAgentWebsocket.connect('', 'component_hub');

    this.loading = false;
  },
  provide() {
    return {
      client: computed(() => this.client),
      componenthubClient: computed(() => this.componenthubClient),
    };
  },
});
</script>
