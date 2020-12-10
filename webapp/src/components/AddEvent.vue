<template>
  <div class="modal fade" id="addEvent" tabindex="-1" role="dialog" aria-labelledby="AddEventLabel" aria-hidden="true">
    <div class="modal-dialog modal-xl">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title" id="exampleModalLongTitle">Add an Event</h5>
          <button type="button" class="close" data-dismiss="modal" aria-label="Close">
            <span aria-hidden="true">&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <form>
            <div class="form-group row">
              <label for="eventName" class="col-sm-2 col-form-label">Event Name</label>
              <div class="col-sm-10">
                <input ref="name" v-model.trim="event.name" type="text" class="form-control" id="eventName" placeholder="Marathon Paris" >
              </div>
            </div>
            <div class="form-group row">
              <label for="localisation" class="col-sm-2 col-form-label">Localisation</label>
              <div class="col-sm-10">
                <input ref="localisation" v-model.trim="event.localisation" type="text" class="form-control" id="localisation" placeholder="Paris France" >
              </div>
            </div>
            <div class="form-group row">
              <label for="eventUrl" class="col-sm-2 col-form-label">Event link/site</label>
              <div class="col-sm-10">
                <input ref="link" v-model.trim="event.event_link" type="url" class="form-control" id="eventUrl" placeholder="https://www.schneiderelectricparismarathon.com" >
                <div class="invalid-feedback text-left">
                  Link should start by http:// or htts:// and contains a . (i.e: https://www.france.fr)
                </div>
              </div>
            </div>
            <div class="form-group row">
              <label for="eventDate" class="col-sm-2 col-form-label">Event Date</label>
              <div class="col-sm-4">
                <input ref="date" v-model.trim="event.event_date" type="date" class="form-control" id="eventDate" >
                <div class="invalid-feedback text-left">
                  The date of the event should be in the future
                </div>
              </div>
              <label for="eventType" class="col-sm-2 col-form-label">Event Type</label>
              <div class="col-sm-4">
                <select ref="type" v-model="event.event_type" class="form-control" id="eventType" >
                  <option v-for="evType in eventTypes" :key="evType" :value="evType">{{ evType }}</option>
                </select>
              </div>
            </div>
          </form>

          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" data-dismiss="modal">Close</button>
<!--            <button type="submit" @click="submit" class="btn btn-primary">Create Event</button>-->
            <button v-if="loading" type="submit" class="btn btn-primary">
              <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
              Creating...
            </button>
            <button v-else type="submit" @click="submit" class="btn btn-primary">
              Create Event
            </button>
          </div>
        </div>

      </div>
    </div>
  </div>
</template>

<script lang="ts">
/* eslint-disable @typescript-eslint/camelcase */

import { Options, Vue } from 'vue-class-component';
import $ from "jquery";
import {Modal} from "bootstrap";
import * as Api from "@/api";
import _ from 'lodash';
import {getAppContext} from "@/main";

@Options({
  emits: ['close'],
  data: () => {
    return {
      eventTypes: [],
      loading: false,
      event: Api.newEvent(getAppContext().user!)
    }
  },
  methods: {
    eventTypeToSvgIcon(eventType: string): string {
      return Api.eventTypeToSvgIconPath(eventType);
    },
    async submit(ev: MouseEvent) {
      ev.preventDefault();
      let isValid = true;
      const event: Api.Event = this.event;

      if(_.isEmpty(event.name)) {
        this.$refs.name.classList.remove("is-valid");
        this.$refs.name.classList.add("is-invalid");
        isValid = false;
      }

      if(_.isEmpty(event.localisation)) {
        this.$refs.localisation.classList.remove("is-valid");
        this.$refs.localisation.classList.add("is-invalid");
        isValid = false;
      }

      if(_.isEmpty(event.event_link) || this.$refs.link.classList.contains("is-invalid")) {
        this.$refs.link.classList.remove("is-valid");
        this.$refs.link.classList.add("is-invalid");
        isValid = false;
      }

      if(_.isEmpty(event.event_date) || this.$refs.date.classList.contains("is-invalid")) {
        this.$refs.date.classList.remove("is-valid");
        this.$refs.date.classList.add("is-invalid");
        isValid = false;
      }

      if(!isValid) {
        return;
      }

      event.event_date = new Date(event.event_date).getTime() / 1000;
      this.loading = true;
      Api.addEvent(event).then((response) => {
        window.location.hash = '#findEvent=' + encodeURI(response.data.name);
        window.location.pathname = '/event/' + response.data.id;
      }).catch((err) => {
        console.log('failure');
        console.log(err);
      });


    }
  },
  async created() {
    this.eventTypes = (await Api.getEventTypes()).data;
  },
  mounted() {
    const el = $('#addEvent');
    const modal = new Modal(el[0]);
    el.on('hidden.bs.modal', () => this.$emit('close'));
    modal.show();
  },
  updated() {
    const event: Api.Event = this.event;
    if (!_.isEmpty(event.name)) {
      this.$refs.name.classList.remove("is-invalid");
      this.$refs.name.classList.add("is-valid");
    }

    if (!_.isEmpty(event.localisation)) {
      this.$refs.localisation.classList.remove("is-invalid");
      this.$refs.localisation.classList.add("is-valid");
    }

    const link = event.event_link;
    if (!_.isEmpty(link)) {
      if (link.match(/^https?:\/\/[^.]+\..+$/)) {
        this.$refs.link.classList.remove("is-invalid");
        this.$refs.link.classList.add("is-valid");
      } else {
        this.$refs.link.classList.remove("is-valid");
        this.$refs.link.classList.add("is-invalid");
      }
    }

    if(!_.isEmpty(event.event_date)) {
      const date = new Date(event.event_date);
      if(date <= new Date()) {
        this.$refs.date.classList.remove("is-valid");
        this.$refs.date.classList.add("is-invalid");
      } else {
        this.$refs.date.classList.remove("is-invalid");
        this.$refs.date.classList.add("is-valid");
      }
    }
  }
})
export default class AddEvent extends Vue {
}
</script>




<style scoped>

</style>