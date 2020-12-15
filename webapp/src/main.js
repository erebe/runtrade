import { createApp } from 'vue';
import App from './App.vue';
import 'bootstrap/dist/css/bootstrap.css';
import 'bootstrap-vue/dist/bootstrap-vue.css';
import Keycloak from "keycloak-js";
const keycloak = Keycloak({ url: 'https://keycloak.erebe.dev/auth', realm: 'runtrade-dev', clientId: 'webapp' });
window.app = { keycloak: keycloak, user: null, selectedEvent: null, event: null };
export function getAppContext() {
    return window.app;
}
const app = createApp(App);
app.mixin({
    methods: {
        keycloak() {
            return keycloak;
        },
    }
});
app.mount('#app');
//# sourceMappingURL=main.js.map