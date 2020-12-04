<template>
  <!--    STEP III-->
  <div class="card-deck mb-4 text-center" id="tradesContainer">
    <div class="card mb-4 shadow-sm">

      <div class="card-header d-sm-inline-flex justify-content-sm-between">
        <span class="col-2 d-none d-md-inline-flex"></span>
        <span>
          <h4 class="my-sm-0 font-weight-normal">
            <span><img src="/images/go.png"></span>  Trade
          </h4>
        </span>
        <span class="mx-0 px-0">
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
          <tr v-for="inscription in inscriptions" :key="inscription.id" :class="(inscription[0].intent === 'Buy') ? 'table-primary': 'table-success'">
            <td>{{ inscription[1].name }}</td>
            <td>
              <i :data-feather="(inscription[0].intent === 'Buy') ? 'crosshair': 'dollar-sign'"></i> {{ inscription[0].intent }}
            </td>
            <td>{{ inscription[2].name }}</td>
            <td>{{ inscription[0].distance }}</td>
            <td>{{ inscription[0].gender }}</td>
            <td>{{ inscription[2].event_type}}</td>
            <td>{{ inscription[0].price }} €</td>
            <td style="width: 150px" class="text-justify">{{ inscription[0].note }}</td>
          </tr>

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

      this.displaySelectEvent = true;
      this.displayChoseTrade = true;

      document.getElementById("tradesContainer")?.scrollIntoView({behavior: "smooth"});

    },
    init() {
      feather.replace();
      this.datatable = $('#trades').DataTable({pageLength: 50, deferRender: true});
      document.getElementById("tradesContainer")?.scrollIntoView({behavior: "smooth"});
    }
  },
  mounted() {
    console.log(this.inscriptions)
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
