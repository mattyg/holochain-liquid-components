import { defineCustomElement } from 'vue';
import CEComponentHubSearch from './ComponentHubSearch.vue';
import CEComponentRenderer from './ComponentRenderer.vue';
import CELiquidComponentBasin from './LiquidComponentBasin.vue';

const ComponentHubSearch = defineCustomElement(CEComponentHubSearch);
const ComponentRenderer = defineCustomElement(CEComponentRenderer);
const LiquidComponentBasin = defineCustomElement(CELiquidComponentBasin);

// export individual elements
export { 
  ComponentHubSearch,
  ComponentRenderer,
  LiquidComponentBasin,
}

export function register() {
  customElements.define('component-hub-search', ComponentHubSearch);
  customElements.define('component-renderer', ComponentRenderer);
  customElements.define('liquid-component-basin', LiquidComponentBasin);
}
