import { createApp, defineCustomElement } from 'vue'
import './style.css'
import App from './App.vue'

/* @vite-ignore */
// @ts-ignore
import { ComponentHubSearch, ComponentRenderer, LiquidComponentBasin, register } from '../../../../components/dist/liquid-components';
register();

createApp(App).mount('#app')
