<template>
  <!--    STEP III-->
  <div class="card-deck mb-4 text-center" id="tradesContainer">
    <div class="card mb-4 shadow-sm">

      <div class="card-header d-sm-inline-flex justify-content-sm-between">
        <span class="col-sm-2 col-0"></span>
        <span>
          <h4 class="my-sm-0 font-weight-normal">
            <span><img src="images/go.png"></span>  Trade
          </h4>
        </span>
        <span class="mx-0 px-0 d-inline-flex align-items-center">
<!--          <a class="d-inline-flex mr-3"><span data-feather="link"></span>  Link</a>-->
          <button type="button" class="btn btn btn-block btn-primary mx-0"
                  data-container="body" data-toggle="popover" data-placement="right" data-html="true"
                  data-content="You want to trade your inscription ? Or propose to buy one ?<br/> Click and add yours !">
            <span data-feather="plus"></span> Add your Trade
          </button>
        </span>
      </div>

      <div class="card-body table-responsive-sm">
        <table class="table table-sm table-hover " id="trades" data-order='[[ 1, "desc" ], [4, "desc"], [3, "asc"], [6, "asc"]]'>
          <thead>
          <tr>
            <th scope="col">Contact</th>
            <th scope="col">Intent</th>
            <th scope="col">Event</th>
            <th scope="col">Distance</th>
            <th scope="col">Gender</th>
            <th scope="col">Type</th>
            <th scope="col">Price</th>
            <th scope="col">Note</th>
          </tr>
          </thead>
          <tbody>
          <tr class="table-primary">
            <td>Sandrine</td>
            <td><i data-feather="crosshair"></i>  Buy</td>
            <td>Marathon Valencia</td>
            <td>42Km</td>
            <td>Women</td>
            <td>Running</td>
            <td>64€</td>
            <td>v</td>
          </tr>
          <tr class="table-success">
            <td>Theo</td>
            <td><i data-feather="dollar-sign"></i>  Sell</td>
            <td>Marathon Valencia</td>
            <td>10Km</td>
            <td>Women</td>
            <td>Running</td>
            <td>32€</td>
            <td>v</td>
          </tr>
          <tr class="table-primary"><td>Richard</td> <td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td><td>42Km</td> <td>Man</td>  <td>Running</td> <td>33€</td> <td></td></tr>
          <tr class="table-primary"><td>Richard</td> <td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td><td>42Km</td> <td>Man</td>  <td>Running</td> <td>33€</td> <td></td></tr>
          <tr class="table-primary"><td>Richard</td> <td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td><td>42Km</td> <td>Man</td>  <td>Running</td> <td>33€</td> <td></td></tr>
          <tr class="table-primary"><td>Richard</td> <td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td><td>42Km</td> <td>Man</td>  <td>Running</td> <td>33€</td> <td></td></tr>
          <tr class="table-primary"><td>Richard</td> <td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td><td>42Km</td> <td>Man</td>  <td>Running</td> <td>33€</td> <td></td></tr>
          <tr class="table-primary"><td>Richard</td> <td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td><td>42Km</td> <td>Man</td>  <td>Running</td> <td>36€</td> <td></td></tr>
          <tr class="table-primary"><td>Richard</td> <td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td><td>42Km</td> <td>Man</td>  <td>Running</td> <td>35€</td> <td></td></tr>
          <tr class="table-primary"><td>Richard</td> <td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td><td>42Km</td> <td>Man</td>  <td>Running</td> <td>32€</td> <td></td></tr>
          <tr class="table-primary"><td>Richard</td> <td><i data-feather="crosshair"></i>  Buy</td> <td>Maratdon Valencia</td><td>42Km</td> <td>Man</td>  <td>Running</td> <td>30€</td> <td></td></tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import Axios from 'axios';
import {AxiosError} from 'axios';
import $ from "jquery";
import _ from 'lodash';
import 'datatables.net'
import 'datatables.net-bs4/js/dataTables.bootstrap4.min'
import 'datatables.net-bs4/css/dataTables.bootstrap4.min.css'
import feather from 'feather-icons'
import {Popover} from "bootstrap";

@Options({
  props: {
    inscriptions: Array
  },
  data: () => {
    return {
      datatable: null,
    }
  },
  methods: {
    eventSelected(ev: MouseEvent) {
      console.log((ev.target as HTMLTableCellElement).dataset.eventid);

      this.trades = [12];

      this.displaySelectEvent = true;
      this.displayChoseTrade = true;

      document.getElementById("tradesContainer")?.scrollIntoView({behavior: "smooth"});

    },
    formatDate(timestamp: number) {
      const date = new Date(timestamp * 1000);
      return '' + date.getFullYear() + '-' + date.getMonth() + '-' + date.getDay();
    },

    init() {
      feather.replace();
      this.datatable = $('#trades').DataTable({pageLength: 50, deferRender: true});
      document.getElementById("tradesContainer")?.scrollIntoView({behavior: "smooth"});
    }
  },
  mounted() {
    this.init();
    $('#tradesContainer [data-toggle="popover"]').get().forEach((el) => {
      new Popover(el, {trigger: "hover"});
    });
  },

  beforeUpdate() {
    this.datatable.destroy();
    this.datatable = null;
  },
  updated() {
    this.init();
  }
})
export default class Trade extends Vue {
  inscriptions!: Array<any>
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>

.card-header h4 {
  width: 100%;
}
</style>
