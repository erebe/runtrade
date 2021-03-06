<template>
  <div class="d-flex flex-column flex-md-row align-items-center p-1 px-md-4 mb-3 bg-white border-bottom shadow-sm">
      <span class="mr-2">
          <img width="60px" height="60px" src="/favicon.png" id="logo">
      </span>
    <h5 class="my-0 mr-md-auto font-weight-normal">RunTrade </h5>
    <span id="signin-logo">
      <button class="m-0 p-0 border-0"
           data-container="body" data-toggle="popover" data-html="true" data-placement="left"
           data-content="Design assets are thanks to <br/>
      <a class='text-nowrap' href='https://www.freepik.com/vectors/nature'>Nature vector created by pch.vector</a><br/>
      <a class='text-nowrap' href='https://www.freepik.com/vectors/people'>People vector created by pch.vector</a><br/>
      <a class='text-nowrap' href='https://www.freepik.com/vectors/medical'>Medical vector created by freepik</a><br/>
      <a class='text-nowrap' href='https://www.freepik.com/vectors/background'>Background vector created by tartila</a><br/>
      <a class='text-nowrap' href='https://www.freepik.com/vectors/sport'>Sport vector created by macrovector</a><br/>
      <a class='text-nowrap' href='https://www.freepik.com/vectors/banner'>Banner vector created by katemangostar</a><br/>
      <a class='text-nowrap' href='https://fr.freepik.com/vectors/nature'>Nature vecteur créé par freepik</a><br/>
      <a class='text-nowrap' href='https://fr.freepik.com/vectors/nouvel-an'>Nouvel an vecteur créé par freepik</a><br/>
      "><img height="55px" width="100px" src="/images/header.webp"></button>
    </span>
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
  <div class="d-none d-lg-block" id="snail"><img src="/images/snail.webp"/></div>
  <div class="d-none d-lg-block" id="bird"><img src="/images/bird.webp"/></div>
  <div class="d-none d-lg-block" id="squirrel"><img src="/images/squirrel.webp"/></div>
  <div class="d-none d-lg-block" id="finish"><img src="/images/finish.webp"/></div>
  <div class="d-none d-lg-block" id="mole" ><img src="/images/mole.webp"/></div>
  <div class="d-none d-lg-block" id="ballon"><img src="/images/ballon.webp"/></div>
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
import $ from "jquery";
import {Popover} from "bootstrap";

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
  mounted() {
    $('#signin-logo [data-toggle="popover"]').get().forEach((el) => {
      (el as any).popover = new Popover(el, {trigger: "focus"});
    });
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
  animation: 120s linear 1s 1 running slidein;
  overflow: hidden;
  right: 105%;
  bottom: 0;
  z-index: -1;
}

@keyframes slidein {
  from { right: 105%; bottom:0;}
  to   { right: 0; bottom:0;}
}

#bird {
  position: absolute;
  animation: 3s linear 30s 1 running fly forwards;
  right: 2%;
  bottom: 72%;
  opacity: 0;
  z-index: -1;
}

@keyframes fly {
  0% { right: 0; bottom: 100%; opacity: 100;}
  25% { right: 5%; opacity: 100}
  50% { right: 0; opacity: 100 }
  75% { right: 5%; opacity: 100}
  100%   { right: 2%; bottom: 72%; opacity: 100 }
}

#squirrel {
  position: absolute;
  animation: 2s linear 18s 1 running appear forwards;
  left: -5%;
  bottom: 8%;
  z-index: -1;
}
@keyframes appear {
  from { left: -5%; bottom: 8%; }
  to  { left: 0; bottom: 8%; }
}

#finish {
  position: absolute;
  right: 0;
  bottom: 0;
  z-index: -1;
}

#mole {
  position: absolute;
  right: 25%;
  bottom: 0;
  animation: 2s linear 100s 1 running mole forwards;
  opacity: 0;
  z-index: -1;
}

@keyframes mole {
  from {  bottom: -10%; opacity: 0;}
  to   {  bottom: 0; opacity: 100;}
}

#ballon {
  position: absolute;
  right: 25px;
  bottom: 50px;
  opacity: 0;
  z-index: -1;
  animation: 3s linear 120s 1 running ballon;
}

@keyframes ballon {
  from {  bottom: 50px; opacity: 0;}
  25% {  opacity: 100;}
  75% {  opacity: 100;}
  to  {  bottom: 400px; opacity: 0;}
}
</style>
