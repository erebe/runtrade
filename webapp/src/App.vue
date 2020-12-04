<template>
  <div class="d-flex flex-column flex-md-row align-items-center p-1 px-md-4 mb-3 bg-white border-bottom shadow-sm">
      <span class="mr-2">
          <img src="/favicon.png" id="logo">
      </span>
    <h5 class="my-0 mr-md-auto font-weight-normal">RunTrade </h5>
    <span id="signin-logo"><img src="/images/header.jpg"></span>
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
    <!--    STEP I-->
    <SearchEvent :eventSearch="extractEventFromUrl()" v-model:events="this.events"></SearchEvent>

    <!--    STEP II-->
    <SelectEvent v-if="displaySelectEvent" :events="events"
                 v-model:selectedEvent="this.selectedEvent"
                 v-model:inscriptions="this.inscriptions"
    ></SelectEvent>

    <!--    STEP III-->
    <Trade v-if="displayTrade" :inscriptions="this.inscriptions"></Trade>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import SearchEvent from "@/components/SearchEvent.vue";
import SelectEvent from "@/components/SelectEvent.vue";
import Trade from "@/components/Trade.vue";
import _ from 'lodash';

@Options({
  components: {
    SearchEvent,
    SelectEvent,
    Trade
  },
  data() {
    return {
      logged: false,
      userProfile: {},

      events: [],
      selectedEvent: null,
      inscriptions: [],

      displaySelectEvent: false,
      displayTrade: false,
    };
  },
  methods: {
    extractEventFromUrl() {
      const hash = window.location.hash.substr(1);
      const result = hash.split('&').reduce(function (res: any, item) {
        const parts = item.split('=');
        res[parts[0]] = parts[1];
        return res;
      }, {});

      const search = (result as any).findEvent;
      if(!_.isUndefined(search)) {
        return decodeURI(search);
      } else {
        return "";
      }
    },
  },
  beforeMount() {
    this.keycloak().init({onLoad: "check-sso"}).then((auth: boolean) => {
      if (this.keycloak().authenticated) {
        this.keycloak().loadUserInfo().then((user: any) => {
          this.userPofile = user;
          this.userProfile.picture = JSON.parse((user as any).picture)['url'];
          this.logged = true;
        })
      }
    });
  },
  watch: {
    events: function (newVal, _oldVal) {
        this.displaySelectEvent = true;
        this.displayTrade = false;
    },

    inscriptions: function (newVal, _oldVal) {
      if(!_.isEmpty(newVal)) {
        this.displayTrade = true;
        history.pushState(null, "RunTrade", '/event/' + this.selectedEvent + window.location.hash)
      }
    }
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
