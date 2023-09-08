import { createWebHashHistory,createRouter } from "vue-router";
import HelloWorld from "../components/HelloWorld.vue";
import Greet from "../components/Greet.vue";
import Greet1 from "../components/Greet1.vue";
const  router = createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            path: '/',
            component: Greet,
            name: 'Greet',
        },
        {
            path: '/Greet1',
            component: Greet1,
            name: 'Greet1',
        }
    ]
})

export  default router;