import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import router from "./router/router.js";
import VXETable from 'vxe-table'
import 'vxe-table/lib/style.css'
createApp(App).use(router).use(VXETable).mount("#app");
