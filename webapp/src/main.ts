
import { createApp } from 'vue'
import App from './App.vue'

import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'
import Keycloak, {KeycloakInstance} from "keycloak-js";

const keycloak = Keycloak({url: 'https://keycloak.erebe.dev/auth', realm: 'runtrade-dev', clientId: 'webapp'});
const app = createApp(App);

app.mixin({
    methods: {
        keycloak(): KeycloakInstance {
            return keycloak;
        }
    }
})
app.mount('#app')

