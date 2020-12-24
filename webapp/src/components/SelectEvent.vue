<template>
  <!--    STEP II-->
  <div id="eventsContainer" class="card-deck mb-4 text-center">
    <div class="card mb-4 shadow-sm">

      <div class="card-header d-sm-inline-flex justify-content-sm-between">
        <span class="col-sm-2 d-none d-md-inline-flex">
        </span>
        <span class="pt-1">
          <h4 class="my-sm-0 font-weight-normal">
            <span><img width="37" height="29" src="/images/steady.webp"></span> Choose your Event
          </h4>
        </span>
        <span class="m-0 px-0">
          <button class="btn btn btn-block btn-primary" data-container="body" data-content="Your Event is not here ? <br/> Feel free to add it by providing few information !"
                  data-html="true" data-placement="right" type="button"
                  v-on:click="createEvent"
          >
            <span data-feather="plus"></span> Add an Event
          </button>

          <!--          Modal for adding event-->
          <AddEvent v-if="displayAddEvent" @close="displayAddEvent = false"></AddEvent>

        </span>
      </div>

      <div class="card-body table-responsive-sm">
        <span v-if="retrievingEvent" class="border d-flex justify-content-center align-items-center"
              style="position:absolute; width: 100%; height: 60%; background-color: rgba(232, 240, 254, 0.9); left: 0%; top: 28%; z-index: 999">
          <div class="spinner-border text-primary" role="status">
            <span class="sr-only">Loading...</span>
          </div>
        </span>
        <table id="events" class="table table-sm table-bordered table-hover" data-order='[[3, "asc"], [1, "asc"]]'>
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
          <tr v-for="event in events" v-bind:key="event.id" v-on:click="eventSelected">
            <td v-bind:data-eventid="event.id">
              <img class="icon-event-type"
                   v-bind:src="eventTypeToSvgIcon(event.event_type)"
                   v-bind:title="event.event_type"/>
              <span class="d-none">{{ event.event_type }}</span>
            </td>
            <td v-bind:data-eventid="event.id">{{ capitalize(event.name) }}</td>
            <td v-bind:data-eventid="event.id">{{ capitalize(event.localisation) }}</td>
            <td v-bind:data-eventid="event.id">{{ formatDate(event.event_date) }}</td>
            <td v-bind:data-eventid="event.id">
              <a rel="noopener noreferrer" target="_blank" v-bind:href="event.event_link">Official website</a>
            </td>
          </tr>

          </tbody>
        </table>
      </div>
    </div>
  </div>

</template>

<script lang="ts">
import {Options, Vue} from 'vue-class-component';
import AddEvent from "@/components/AddEvent.vue";
import $ from "jquery";
import {Popover} from "bootstrap";
import 'datatables.net-bs4'
import 'datatables.net-bs4/css/dataTables.bootstrap4.min.css'
import feather from 'feather-icons'
import * as Api from "@/api";
import _ from "lodash";
import {KeycloakInstance} from "keycloak-js";
import { PropType } from 'vue';
import {getAppContext} from "@/main";

@Options({
  components: {
    AddEvent
  },
  props: {
    events: Object as PropType<Api.Event[]>,
  },
  emits: ['update:event', 'update:inscriptions'],
  data: () => {
    return {
      datatable: null,
      retrievingEvent: false,
      displayAddEvent: false,
      selectedEvent: null,
    }
  },
  methods: {
    capitalize(str: string): string {
      return _.capitalize(str);
    },
    async createEvent(ev: MouseEvent) {
      ev.preventDefault();
      const keycloak = this.keycloak() as KeycloakInstance;
      if (!keycloak.authenticated || _.isNil(getAppContext().user)) {
        keycloak.login();
      }
      this.displayAddEvent = true;
    },
    async eventSelected(ev: MouseEvent | null) {
      if(_.isNil(ev) && this.events.length > 1) {
        return;
      }

      // do nothing if user click on a link
      if ((ev?.target as HTMLElement).nodeName == "A") {
        return;
      }

      ev?.preventDefault();
      const eventId = _.isNil(ev)
          ? this.events[0].id
          : parseInt((ev.target as HTMLTableCellElement).dataset.eventid!);
      this.retrievingEvent = true;
      try {
        // Find events
        const inscriptions = await Api.getInscriptionForEvent(eventId)
        this.$emit('update:event', _.find(this.events, ev => ev.id === eventId));
        this.$emit('update:inscriptions', inscriptions.data);

      } catch (error) {
        console.error(error);
      }
      this.retrievingEvent = false;
    },
    formatDate(timestamp: number) {
      const months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
      const date = new Date(timestamp * 1000);
      //return '' + date.getDay() + ' ' + months[date.getMonth()] + ' ' + date.getFullYear();
      return '' + date.getUTCFullYear() + '-'
          + ('0' + (date.getUTCMonth() + 1)).slice(-2) + '-'
          + ('0' + (date.getUTCDate())).slice(-2);
    },
    eventTypeToSvgIcon(eventType: string) {
      return Api.eventTypeToSvgIconPath(eventType)
    },
    init() {
      feather.replace();
      this.datatable = $('#events').DataTable({pageLength: 10, deferRender: true});
    }
  },
  async mounted() {
    this.init();
    $('#eventsContainer [data-toggle="popover"]').get().forEach((el) => {
      new Popover(el, {trigger: "hover"});
    });
    await this.eventSelected(null);
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
  events!: Array<Api.Event>
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
