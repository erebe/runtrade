<template>
  <div class="d-flex flex-column flex-md-row align-items-center p-1 px-md-4 mb-3 bg-white border-bottom shadow-sm">
      <span class="mr-2">
          <img src="favicon.png" id="logo">
      </span>
    <h5 class="my-0 mr-md-auto font-weight-normal">RunTrade </h5>
    <span id="signin-logo"><img src="images/header.jpg"></span>
    <nav class="my-2 my-md-0 mr-md-3">
<!--      <a class="p-2 text-dark" href="#">Your trades</a>-->
    </nav>
    <img v-if="logged" id="userPicture" v-bind:src="userProfile.picture">
    <a v-else class="btn btn-primary" id="btn-sign" v-on:click="keycloak.login()" href="#">Sign In</a>
  </div>

  <!--<div class="pricing-header px-3 py-3 pt-md-5 pb-md-4 mx-auto text-center">-->
  <!--  <h1 class="display-4">Pricing</h1>-->
  <!--  <p class="lead">Quickly build an effective pricing table for your potential customers with this Bootstrap example. It’s built with default Bootstrap components and utilities with little customization.</p>-->
  <!--</div>-->

  <div class="container">
    <SearchEvent event=""></SearchEvent>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import SearchEvent from "@/components/SearchEvent.vue";
import Keycloak from 'keycloak-js'

const keycloak = Keycloak({url: 'https://keycloak.erebe.dev/auth', realm: 'runtrade-dev', clientId: 'webapp'});
(window as any) .keycloak = keycloak;

@Options({
  components: {
    SearchEvent,
  },
  data() {
    return {
      logged: false,
      userProfile: {},
      keycloak: keycloak
    };
  },
  beforeMount() {
    keycloak.init({onLoad: "check-sso"}).then((auth) => {
      if (keycloak.authenticated) {
        keycloak.loadUserInfo().then((user) => {
          this.userPofile = user;
          this.userProfile.picture = JSON.parse((user as any).picture)['url'];
          this.logged = true;
        })
      }
    });
  }
})
export default class App extends Vue {}
</script>

<style>
html {
  min-height: 100%;
}

#logo {
  width: 60px;
}

#userPicture {
  width: 50px;
  height: 50px;
  border-radius: 50%;
}
</style>
