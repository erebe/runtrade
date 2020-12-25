<template>
  <div class="d-flex flex-column flex-md-row align-items-center p-1 px-md-4 mb-3 bg-white border-bottom shadow-sm">
      <span class="mr-2">
          <img width="60px" height="60px" src="/favicon.png" id="logo">
      </span>
    <h5 class="my-0 mr-md-auto font-weight-normal">RunTrade </h5>
    <span id="signin-logo"><img height="55px" width="99px" src="/images/header.webp"></span>
    <nav class="my-2 my-md-0 mr-md-3">
<!--      <a class="p-2 text-dark" href="#">Your trades</a>-->
    </nav>
    <img v-if="logged" id="userPicture" width="50px" height="50px" v-bind:src="userProfile.picture">
    <a v-else class="btn btn-primary" id="btn-sign" v-on:click="keycloak().login()" href="#">Sign In</a>
  </div>

  <!--<div class="pricing-header px-3 py-3 pt-md-5 pb-md-4 mx-auto text-center">-->
  <!--  <h1 class="display-4">Pricing</h1>-->
  <!--  <p class="lead">Quickly build an effective pricing table for your potential customers with this Bootstrap example. It’s built with default Bootstrap components and utilities with little customization.</p>-->
  <!--</div>-->

  <div class="container">
    <!--    STEP I-->
    <SearchEvent v-model:searchEvent="searchEvent" v-model:events="events"></SearchEvent>

    <!--    STEP II-->
    <SelectEvent v-if="displaySelectEvent"
                 :events="events"
                 v-model:event="event"
                 v-model:inscriptions="inscriptions"
    ></SelectEvent>

    <!--    STEP III-->
    <Trade v-if="displayTrade" :event="event" v-model:inscriptions="inscriptions" :user="user"></Trade>
  </div>
  <div id="snail" class="d-none d-md-block"><img src="/images/snail.png"/></div>
  <div id="bird" class="d-none d-md-block"><img src="/images/bird.png"/></div>
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
      user: null as unknown as Api.User,

      searchEvent: null,
      events: null as unknown as Array<Api.Event>,
      event: null as unknown as Api.Event,
      inscriptions: null as unknown as Array<[Api.Inscription, Api.User, Api.Event]>,

      displaySelectEvent: false,
      displayTrade: false,
    };
  },
  methods: {
    async initStateFromUrl() {
      const hash = window.location.hash.substr(1);
      const result = hash.split('&').reduce(function (res: any, item) {
        const parts = item.split('=');
        res[parts[0]] = parts[1];
        return res;
      }, {});

      const search = (result as any).findEvent;

      const event = (result as any).event;
      if(!_.isUndefined(event)) {
        this.event = (await Api.getEvent(parseInt(event))).data;
        return;
      }

      if(!_.isUndefined(search)) {
        this.searchEvent = decodeURI(search);
      }

    },
  },
  created() {
    const token = localStorage.getItem("vue-token");
    const tokenRefresh = localStorage.getItem("vue-refresh-token");
    const keycloak = this.keycloak() as KeycloakInstance;

    keycloak.init({onLoad: "check-sso", token: token!, refreshToken: tokenRefresh!}).then(async (authenticated: boolean) => {
      if(!authenticated) {
        console.log("NOT authenticated");
        localStorage.removeItem("vue-token");
        localStorage.removeItem("vue-refresh-token");
        this.initStateFromUrl();
        return;
      }


      console.log("Authenticated");
      // Check if our Access token is invalid, do a login if it has expired
      if(keycloak.isTokenExpired()) {
        console.log("Token Expired");
        //keycloak.login();
        await keycloak.updateToken(0);
      }

      this.initStateFromUrl();
      localStorage.setItem("vue-token", keycloak.token!);
      localStorage.setItem("vue-refresh-token", keycloak.refreshToken!);

      axios.interceptors.request.use(config => {
        config.headers.Authorization = `Bearer ${this.keycloak().token}`
        return config
      }, error => {
        return Promise.reject(error)
      })

      keycloak.loadUserProfile().then(async user => {
        this.userProfile = user;
        this.userProfile.picture = (user as any).attributes.picture[0];
        this.logged = true;

        // eslint-disable-next-line @typescript-eslint/camelcase
        const UserResponse = await Api.userLogged({id: keycloak.subject!, name: user.username!, email: user.email!, contact: "", last_logged: 0});
        getAppContext().user = UserResponse.data;
        this.user = UserResponse.data;
      }).catch(err => {
        console.error("Cannot retrieve User profile");
        console.error(err);
      });

      setInterval(() => {
        keycloak.updateToken(70).then((success: boolean) => {
          if(success) {
            localStorage.setItem("vue-token", this.keycloak().token);
            localStorage.setItem("vue-refresh-token", this.keycloak().refreshToken);
          }
        }).catch(err => {
          console.error("cannot refresh auth token");
          console.error(err);
      });
      }, 60 * 1000);
    }).catch(err => {
      console.error("Cannot check sso-login");
      console.error(err);
    });
  },
  beforeUpdate() {
    const findEvent = (_.isNil(this.searchEvent) || _.isEmpty(this.searchEvent)) ? '' : 'findEvent=' + encodeURI(this.searchEvent);
    const event = (_.isNil(this.event)) ? '' : 'event=' + this.event.id;
    const path = ('/#' + findEvent + '&' + event)
        .replace(/#&/, '#')
        .replace(/&$/, '');
    history.pushState(null, "Runtrade", path);
  },
  updated() {
    this.displaySelectEvent = (!_.isNil(this.events));
    this.displayTrade = (!_.isNil(this.event) && !_.isNil(this.inscriptions));
  },
  watch: {
    searchEvent: function (newVal, oldVal) {
      this.event = null;
    },
    event: function (newVal, oldVal) {
      if(_.isNil(newVal)) {
        return;
      }
       if(_.isNil(this.events)) {
         this.events = [newVal];
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

#snail {
  position: absolute;
  animation: 120s linear 1s infinite running slidein;
  overflow: hidden;
  right: 105%;
  bottom: 0;
}

@keyframes slidein {
  from { right: 105%; bottom:0;}
  to   { right: 0; bottom:0;}
}

#bird {
  position: absolute;
  animation: 3s linear 30s 1 running fly forwards;
  overflow: hidden;
  right: 0;
  bottom: 100%;
}

@keyframes fly {
  0% { right: 0%; bottom: 100%;}
  25% { right: 5%; }
  50% { right: 0%; }
  75% { right: 5%; }
  100%   { right: 2%; bottom: 72%; }
}
</style>
