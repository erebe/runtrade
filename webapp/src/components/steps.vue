<template>
  <!--    STEP I-->
  <div v-if="displayFindEvent" class="card-deck mb-4 text-center">
    <div class="card mb-4 shadow-sm">
      <div class="card-header">
        <h4 class="my-0 font-weight-normal"><span><img src="images/ready.png"></span>  Find your Event</h4>
      </div>
      <div class="card-body">
        <form v-on:submit="findEvent">
          <div class="form-group row">
            <label for="runNameSearch" class="col-sm-2 col-form-label pr-0">Event Name</label>
            <div class="col-sm-8">
              <input v-model.trim="eventName" type="text" class="form-control" id="runNameSearch" placeholder="Name of your event (i.e: Marathon Valencia)">
            </div>

            <!-- SEARCH BUTTON-->
            <div v-if="searchingEvent" class="col-sm-2">
              <button type="submit" class="btn btn btn-block btn-primary">
                <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
                Searching...
              </button>
            </div>
            <div v-else class="col-sm-2">
              <button type="submit" class="btn btn btn-block btn-primary">
                Search
              </button>
            </div>
            <!-- SEARCH BUTTON-->

          </div>
        </form>
      </div>
    </div>
  </div>

  <!--    STEP II-->
  <div v-if="displaySelectEvent" class="card-deck mb-4 text-center" id="eventsContainer">
    <div class="card mb-4 shadow-sm">
      <div class="card-header">
        <h4 class="my-0 font-weight-normal"><span><img src="images/steady.png"></span> Choose your Event</h4>
      </div>
      <div class="card-body table-responsive-sm">
        <table class="table table-sm table-bordered table-hover" id="events" data-order='[[ 3, "desc" ], [1, "asc"]]'>
          <thead>
          <tr>
            <th scope="col">Type</th>
            <th scope="col">Event</th>
            <th scope="col">Location</th>
            <th scope="col">Date</th>
            <th scope="col">Link</th>
          </tr>
          </thead>
          <tbody>
          <tr v-for="event in events" v-bind:key="event.id" v-on:click="test" >
            <td v-bind:data-eventid="event.id">
                <img class="icon-event-type"
                     data-toggle="tooltip"
                     v-bind:title="event.event_type"
                     v-bind:src="eventTypeToSvgIconPath(event.event_type)"/>
            </td>
            <td v-bind:data-eventid="event.id">{{ event.name }}</td>
            <td v-bind:data-eventid="event.id">{{ event.localisation }}</td>
            <td v-bind:data-eventid="event.id">{{ formatDate(event.event_date) }}</td>
            <td v-bind:data-eventid="event.id"><a v-bind:href="event.event_link">{{ event.event_link }}</a></td>
          </tr>

          </tbody>
        </table>
      </div>
    </div>
  </div>

  <!--    STEP III -->
  <div v-if="displayChoseTrade" class="card-deck mb-4 text-center" id="tradesContainer">
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


@Options({
  props: {
    event: String
  },
  data: () => {
    return {
      displayFindEvent: true,
      searchingEvent: false,
      eventName: "",

      displaySelectEvent: false,
      events: [],
      eventsDataTable: null,

      displayChoseTrade: false,
      trades: [],
      tradesDataTable: null,
    }
  },
  methods: {
    test(ev: MouseEvent) {
      console.log((ev.target as HTMLTableCellElement).dataset.eventid);

      this.trades = [12];

      this.displayFindEvent = true;
      this.displaySelectEvent = true;
      this.displayChoseTrade = true;


      // Reset datatable date
      if(!_.isNil(this.tradesDataTable))
        this.tradesDataTable.destroy();
      this.tradesDataTable = null;
      document.getElementById("tradesContainer")?.scrollIntoView({behavior: "smooth"});

    },
    eventTypeToSvgIconPath(eventType: string) {
      switch (eventType.toLocaleLowerCase()) {
        case 'run': return 'icons/run.svg'
        case 'trail': return 'icons/trail.svg'
        case 'bike': return 'icons/bike.svg'
        default: return 'icons/other.svg'

      }
    },
    formatDate(timestamp: number) {
      const date = new Date(timestamp * 1000);
      return '' + date.getFullYear() + '-' + date.getMonth() + '-' + date.getDay();
    },
    async findEvent(ev: Event) {
      ev.preventDefault();
      console.log(this.eventName);
      this.searchingEvent = true;
      try {
        // Find events
        const response = await Axios.get('http://localhost:8081/api/v1/events/' + encodeURI(this.eventName));
        this.events = response.data;

        // Reset datatable date
        this.eventsDataTable.destroy();
        this.eventsDataTable = null;

        // Transition to new state
        this.searchingEvent = false;
        this.displayFindEvent = true;
        this.displaySelectEvent = true;

      } catch (error) {
        this.searchingEvent = false;
        console.error(error as AxiosError);
      }
    },
  },
  updated() {
    if(_.isNil(this.eventsDataTable)) {
      this.eventsDataTable = $('#events').DataTable({ pageLength: 10, deferRender: true });
    }

    if(_.isNil(this.tradesDataTable)) {
      this.tradeDataTable = $('#trades').DataTable({ pageLength: 50, deferRender: true });
      document.getElementById("tradesContainer")?.scrollIntoView({behavior: "smooth"});
    }
  },
})
export default class Steps extends Vue {
  event!: string
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>

.icon-event-type {
  width: 25px;
}


</style>
