import { createApp } from 'vue'
import App from './App.vue'

import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'
import Keycloak, {KeycloakInstance} from "keycloak-js";
import {User} from "@/api";

const keycloak = Keycloak({
    url: process.env.KEYCLOAK_URL,
    realm: process.env.KEYCLOAK_REALM,
    clientId: process.env.KEYCLOAK_CLIENTID,
});

interface AppContext {
    keycloak: KeycloakInstance;
    user: User | null;
    selectedEvent: number | null;
    event: Event | null;
}

(window as any).app = { keycloak: keycloak, user: null, selectedEvent: null, event: null } as AppContext;
export function getAppContext(): AppContext {
    return (window as any).app;
}

const app = createApp(App);
app.mixin({
    methods: {
        keycloak(): KeycloakInstance {
            return keycloak;
        },
    }
})

app.mount('#app')

