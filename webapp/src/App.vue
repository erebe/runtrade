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
    <a v-else class="btn btn-primary" id="btn-sign" v-on:click="keycloak().login()" href="#">Sign In</a>
  </div>

  <!--<div class="pricing-header px-3 py-3 pt-md-5 pb-md-4 mx-auto text-center">-->
  <!--  <h1 class="display-4">Pricing</h1>-->
  <!--  <p class="lead">Quickly build an effective pricing table for your potential customers with this Bootstrap example. It’s built with default Bootstrap components and utilities with little customization.</p>-->
  <!--</div>-->

  <div class="container">
    <!--    STEP I-->
    <SearchEvent :eventSearch="this.searchEvent" v-model:events="this.events"></SearchEvent>

    <!--    STEP II-->
    <SelectEvent v-if="displaySelectEvent" :events="events"
                 v-model:selectedEvent="this.selectedEvent"
    ></SelectEvent>

    <!--    STEP III-->
    <Trade v-if="displayTrade" :event="this.event" :inscriptions="this.inscriptions"></Trade>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import SearchEvent from "@/components/SearchEvent.vue";
import SelectEvent from "@/components/SelectEvent.vue";
import Trade from "@/components/Trade.vue";
import _ from 'lodash';
import axios from 'axios';
import {KeycloakInstance} from "keycloak-js";
import * as Api from "@/api";
import {getAppContext} from "@/main";

@Options({
  components: {
    SearchEvent,
    SelectEvent,
    Trade
  },
  data() {
    return {
      logged: false,
      userProfile: null,

      searchEvent: null,
      events: null as unknown as Array<Api.Event>,
      selectedEvent: 0,
      event: null as unknown as Api.Event,
      inscriptions: null as unknown as Array<[Api.Inscription, Api.User, Api.Event]>,

      displaySelectEvent: false,
      displayTrade: false,
    };
  },
  methods: {
    initStateFromUrl() {
      const hash = window.location.hash.substr(1);
      const result = hash.split('&').reduce(function (res: any, item) {
        const parts = item.split('=');
        res[parts[0]] = parts[1];
        return res;
      }, {});

      const search = (result as any).findEvent;
      if(!_.isUndefined(search)) {
        this.searchEvent = decodeURI(search);
      }

      const path = window.location.pathname.split('/');
      if (path.length == 3 && path[1] == "event") {
        this.selectedEvent = parseInt(path[2]);
      }

    },
  },

  beforeMount() {
    const token = localStorage.getItem("vue-token");
    const tokenRefresh = localStorage.getItem("vue-refresh-token");
    const keycloak = this.keycloak() as KeycloakInstance;
    console.log(keycloak);
    keycloak.init({onLoad: "check-sso", token: token!, refreshToken: tokenRefresh!}).then((auth: boolean) => {
      if (this.keycloak().authenticated) {
        localStorage.setItem("vue-token", this.keycloak().token);
        localStorage.setItem("vue-refresh-token", this.keycloak().refreshToken);

        axios.interceptors.request.use(config => {
          config.headers.Authorization = `Bearer ${this.keycloak().token}`
          return config
        }, error => {
          return Promise.reject(error)
        })

        keycloak.loadUserProfile().then(user => {
          this.userProfile = user;
          this.userProfile.picture = JSON.parse((user as any).attributes.picture)['url'];
          this.logged = true;

          // eslint-disable-next-line @typescript-eslint/camelcase
          Api.userLogged({name: user.username!, email: user.email!, contact: "", external_id: keycloak.subject!, last_logged: 0})
              .then(response => {
                getAppContext().user = response.data;
              });
        })

        setInterval(() => {
          keycloak.updateToken(70).then((success: boolean) => {
            if(success) {
              localStorage.setItem("vue-token", this.keycloak().token);
              localStorage.setItem("vue-refresh-token", this.keycloak().refreshToken);
            }
          });

        }, 60000)
      }
    });

    this.initStateFromUrl();
  },
  async beforeUpdate() {
    if(!_.isNil(this.events)) {
      this.displaySelectEvent = true;
    }

    this.displayTrade = (this.selectedEvent !== 0);
  },
  watch: {
    events: function() {
      this.selectedEvent = 0;
    },
    selectedEvent: async function (eventId, _oldVal) {
      if (eventId === 0) {
        history.pushState(null, "RunTrade", '/' + window.location.hash)
        return;
      }

      history.pushState(null, "RunTrade", '/event/' + eventId + window.location.hash)
      try {
        const [inscriptions, event] = await Promise.all([Api.getInscriptionForEvent(eventId), Api.getEvent(eventId)]);
        this.inscriptions = inscriptions.data;
        this.event = event.data;
      } catch (err) {
        console.error(err);
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
