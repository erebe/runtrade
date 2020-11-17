<template>
  <img alt="Vue logo" src="./assets/logo.png">
  <HelloWorld msg="Welcome to Your Vue.js + TypeScript App"/>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import HelloWorld from './components/HelloWorld.vue';
import Keycloak from 'keycloak-js'

const keycloak = Keycloak({url: 'https://keycloak.erebe.dev/auth', realm: 'runtrade-dev', clientId: 'webapp'});

keycloak.init({}).then((auth) => {

  console.log("toto" + auth);
  if (!auth) {
    keycloak.login();
  }

});

@Options({
  components: {
    HelloWorld,
  },
})
export default class App extends Vue {}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
