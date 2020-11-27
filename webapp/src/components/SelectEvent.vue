<template>
  <!--    STEP II-->
  <div class="card-deck mb-4 text-center" id="eventsContainer">
    <div class="card mb-4 shadow-sm">

      <div class="card-header d-sm-inline-flex justify-content-sm-between">
        <span class="col-sm-2">
        </span>
        <span class="pt-1">
          <h4 class="my-sm-0 font-weight-normal">
            <span><img src="images/steady.png"></span> Choose your Event
          </h4>
        </span>
        <span class="m-0 px-0">
          <button type="button" class="btn btn btn-block btn-primary"
                  data-container="body" data-toggle="popover" data-placement="right" data-html="true"
                  data-content="Your Event is not here ? <br/> Feel free to add it by providing few information !"
          >
            <span data-feather="plus"></span> Create your Event
          </button>
        </span>
      </div>

      <div class="card-body table-responsive-sm">
        <span v-if="retrievingEvent" class="border d-flex justify-content-center align-items-center"
              style="position:absolute; width: 100%; height: 60%; background-color: rgba(232, 240, 254, 0.9); left: 0%; top: 28%; z-index: 999">
          <div class="spinner-border text-primary" role="status">
            <span class="sr-only">Loading...</span>
          </div>
        </span>
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
          <tr v-for="event in events" v-bind:key="event.id" v-on:click="eventSelected" >
            <td v-bind:data-eventid="event.id">
              <img class="icon-event-type"
                   v-bind:title="event.event_type"
                   v-bind:src="eventTypeToSvgIconPath(event.event_type)"/>
            </td>
            <td v-bind:data-eventid="event.id">{{ event.name }}</td>
            <td v-bind:data-eventid="event.id">{{ event.localisation }}</td>
            <td v-bind:data-eventid="event.id">{{ formatDate(event.event_date) }}</td>
            <td v-bind:data-eventid="event.id">
              <a v-bind:href="event.event_link" target="_blank" rel="noopener noreferrer">{{ event.event_link }}</a>
            </td>
          </tr>

          </tbody>
        </table>
      </div>
    </div>
  </div>

  <Trade v-if="displayChoseTrade" :inscriptions="inscriptions"></Trade>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import Trade from "@/components/Trade.vue";
import Axios from 'axios';
import $ from "jquery";
import {Popover} from "bootstrap";
import 'datatables.net-bs4'
import 'datatables.net-bs4/css/dataTables.bootstrap4.min.css'
import feather from 'feather-icons'
import {getInscriptionForEvent} from "@/api";

@Options({
  components: {
    Trade
  },
  props: {
    events: Array
  },
  data: () => {
    return {
      selectedEvent: null,
      datatable: null,
      retrievingEvent: false,
      displayChoseTrade: false,
      inscriptions: Array
    }
  },
  methods: {
    async eventSelected(ev: MouseEvent) {
      // do nothing if user click on a link
      if((ev.target as HTMLElement).nodeName == "A") {
        return;
      }

      ev.preventDefault();
      this.retrievingEvent = true;
      await new Promise(resolve => setTimeout(resolve, 2000));
      try {
        // Find events
        const eventId = (ev.target as HTMLTableCellElement).dataset.eventid!
        const response = await getInscriptionForEvent(parseInt(eventId))
        this.inscriptions = response.data as Array<any>;

        // Transition to new state
        this.displayChoseTrade = true;

      } catch (error) {
        console.error(error);
      }
      this.displayChoseTrade = true;
      this.inscriptions = [];
      this.retrievingEvent = false;
    },
    eventTypeToSvgIconPath(eventType: string) {
      switch (eventType.toLocaleLowerCase()) {
        case 'run':
          return 'icons/run.svg'
        case 'trail':
          return 'icons/trail.svg'
        case 'bike':
          return 'icons/bike.svg'
        default:
          return 'icons/other.svg'

      }
    },
    formatDate(timestamp: number) {
      const months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
      const date = new Date(timestamp * 1000);
      //return '' + date.getDay() + ' ' + months[date.getMonth()] + ' ' + date.getFullYear();
      return '' + date.getFullYear() + '-' + (date.getMonth() + 1) + '-' + date.getDay();
    },

    init() {
      feather.replace();
      this.datatable = $('#events').DataTable({pageLength: 10, deferRender: true});
    }
  },
  mounted() {
    this.init();
    $('#eventsContainer [data-toggle="popover"]').get().forEach((el) => {
      new Popover(el, {trigger: "hover"});
    });
  }
,
  beforeUpdate() {
    this.datatable.destroy();
    this.datatable = null;
  },
  updated() {
    this.init();
  }
})
export default class SelectEvent extends Vue {
  events!: Array<any>
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>

.icon-event-type {
  width: 25px;
}

.card-header h4 {
  width: 100%;
}

</style>
