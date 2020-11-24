<template>
  <div class="d-flex flex-column flex-md-row align-items-center p-1 px-md-4 mb-3 bg-white border-bottom shadow-sm">
      <span class="mr-2">
          <img src="favicon.png" id="logo">
      </span>
    <h5 class="my-0 mr-md-auto font-weight-normal">RunTrade</h5>
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

    <!--    STEP I-->
    <div class="card-deck mb-4 text-center">
      <div class="card mb-4 shadow-sm">
        <div class="card-header">
          <h4 class="my-0 font-weight-normal"><span><img src="images/ready.png"></span>  Find your Event</h4>
        </div>
        <div class="card-body">
          <form>
            <div class="form-group row">
              <label for="runNameSearch" class="col-sm-2 col-form-label pr-0">Event Name</label>
              <div class="col-sm-8">
                <input type="text" class="form-control" id="runNameSearch" placeholder="Name of your event (i.e: Marathon Valencia)">
              </div>
              <div class="col-sm-2">
                <button type="button" class="btn btn btn-block btn-primary">Search</button>
              </div>
            </div>
          </form>
        </div>
      </div>
    </div>

    <!--    STEP II-->
    <div class="card-deck mb-4 text-center">
      <div class="card mb-4 shadow-sm">
        <div class="card-header">
          <h4 class="my-0 font-weight-normal"><span><img src="images/steady.png"></span> Choose your Event</h4>
        </div>
        <div class="card-body table-responsive-sm">
          <table class="table table-sm table-striped table-bordered table-hover" id="events">
            <thead>
            <tr>
              <th scope="col">Event</th>
              <th scope="col">Type</th>
              <th scope="col">Date</th>
              <th scope="col">Location</th>
            </tr>
            </thead>
            <tbody>
            <tr>
              <td>Valencia</td>
              <td>Running</td>
              <td>03/12/2020</td>
              <td>Spain</td>
            </tr>
            <tr>
              <td>Chataignes</td>
              <td>Trail</td>
              <td>13/11/2020</td>
              <td>Paris</td>
            </tr>
            <tr>
              <td>Semi de Deauville</td>
              <td>Running</td>
              <td>13/11/2020</td>
              <td>Deauville</td>
            </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <!--    STEP III -->
    <div class="card-deck mb-4 text-center">
      <div class="card mb-4 shadow-sm">
        <div class="card-header">
          <h4 class="my-0 font-weight-normal"><span><img src="images/go.png"></span>  Trade</h4>
        </div>

        <div class="card-body table-responsive-sm">
          <table class="table table-sm table-hover " id="trades" data-order='[[ 0, "desc" ], [3, "asc"], [5, "asc"]]'>
            <thead>
            <tr>
              <th scope="col">Intent</th>
              <th scope="col">Event</th>
              <th scope="col">Type</th>
              <th scope="col">Distance</th>
              <th scope="col">Gender</th>
              <th scope="col">Price</th>
              <th scope="col">Contact</th>
            </tr>
            </thead>
            <tbody>
            <tr class="table-primary">
              <td><i data-feather="crosshair"></i>  Buy</td>
              <td>Marathon Valencia</td>
              <td>Running</td>
              <td>42Km</td>
              <td>Women</td>
              <td>64€</td>
              <td>Sandrine</td>
            </tr>
            <tr class="table-success">
              <td><i data-feather="dollar-sign"></i>  Sell</td>
              <td>Marathon Valencia</td>
              <td>Running</td>
              <td>10Km</td>
              <td>Women</td>
              <td>32€</td>
              <td>Theo</td>
            </tr>
            <tr class="table-primary"><td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td> <td>Running</td> <td>42Km</td> <td>Man</td> <td>33€</td> <td>Richard</td> </tr>
            <tr class="table-primary"><td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td> <td>Running</td> <td>42Km</td> <td>Man</td> <td>33€</td> <td>Richard</td> </tr>
            <tr class="table-primary"><td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td> <td>Running</td> <td>42Km</td> <td>Man</td> <td>33€</td> <td>Richard</td> </tr>
            <tr class="table-primary"><td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td> <td>Running</td> <td>42Km</td> <td>Man</td> <td>33€</td> <td>Richard</td> </tr>
            <tr class="table-primary"><td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td> <td>Running</td> <td>42Km</td> <td>Man</td> <td>33€</td> <td>Richard</td> </tr>
            <tr class="table-primary"><td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td> <td>Running</td> <td>42Km</td> <td>Man</td> <td>36€</td> <td>Richard</td> </tr>
            <tr class="table-primary"><td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td> <td>Running</td> <td>42Km</td> <td>Man</td> <td>35€</td> <td>Richard</td> </tr>
            <tr class="table-primary"><td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td> <td>Running</td> <td>42Km</td> <td>Man</td> <td>32€</td> <td>Richard</td> </tr>
            <tr class="table-primary"><td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td> <td>Running</td> <td>42Km</td> <td>Man</td> <td>30€</td> <td>Richard</td> </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import HelloWorld from './components/HelloWorld.vue';
import Keycloak from 'keycloak-js'

const keycloak = Keycloak({url: 'https://keycloak.erebe.dev/auth', realm: 'runtrade-dev', clientId: 'webapp'});
(window as any) .keycloak = keycloak;

@Options({
  components: {
    HelloWorld,
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
