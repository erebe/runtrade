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
                <input v-model="event.name" type="text" class="form-control" id="eventName" placeholder="Marathon Paris">
              </div>
            </div>
            <div class="form-group row">
              <label for="localisation" class="col-sm-2 col-form-label">Localisation</label>
              <div class="col-sm-10">
                <input v-model="event.localisation" type="text" class="form-control" id="localisation" placeholder="Paris France">
              </div>
            </div>
            <div class="form-group row">
              <label for="eventUrl" class="col-sm-2 col-form-label">Event link/site</label>
              <div class="col-sm-10">
                <input v-model="event.link" type="url" class="form-control" id="eventUrl" placeholder="https://www.schneiderelectricparismarathon.com">
              </div>
            </div>
            <div class="form-group row">
              <label for="eventDate" class="col-sm-2 col-form-label">Event Date</label>
              <div class="col-sm-4">
                <input v-model="event.date" type="date" class="form-control" id="eventDate">
              </div>
              <label for="eventType" class="col-sm-2 col-form-label">Event Type</label>
              <div class="col-sm-4">
                <select v-model="event.type" class="form-control" id="eventType">
                  <option v-for="evType in eventTypes" :key="evType" :value="evType">{{ evType }}</option>
                </select>
              </div>
            </div>
          </form>

          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" data-dismiss="modal">Close</button>
            <button type="button" class="btn btn-primary">Create Event</button>
          </div>
        </div>

      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import $ from "jquery";
import {Modal} from "bootstrap";
import {eventTypeToSvgIconPath, getEventTypes} from "@/api";

@Options({
  emits: ['close'],
  data: () => {
    return {
      eventTypes: [],
      event: {
        name: "",
        localisation: "",
        link: "",
        date: "",
        type: "Run",
        createdBy: 0,
      }
    }
  },
  methods: {
    eventTypeToSvgIcon(eventType: string): string {
      return eventTypeToSvgIconPath(eventType);
    }
  },
  async mounted() {
    const el = $('#addEvent');
    const modal = new Modal(el[0]);
    el.on('hidden.bs.modal', () => this.$emit('close'));
    modal.show();
    this.eventTypes = (await getEventTypes()).data;
  }
  ,
  updated() {
    console.log(this.keycloak());
  }
})
export default class AddEvent extends Vue {
}
</script>




<style scoped>

</style>