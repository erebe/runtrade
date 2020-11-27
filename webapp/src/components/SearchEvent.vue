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
  <SelectEvent v-if="displaySelectEvent" :events="events"></SelectEvent>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import SelectEvent from "@/components/SelectEvent.vue";
import "../api"
import {findEventByName} from "@/api";

@Options({
  components: {
    SelectEvent
  },
  props: {
    event: String
  },
  data: () => {
    return {
      displayFindEvent: true,
      searchingEvent: false,
      eventName: "",

      displaySelectEvent: false,
      events: null,
    }
  },
  methods: {
    async findEvent(ev: Event) {
      ev.preventDefault();
      this.searchingEvent = true;
      try {
        // Find events
        const response = await findEventByName(this.eventName)
        this.events = response.data as Array<any>;

        // Transition to new state
        this.displaySelectEvent = true;

      } catch (error) {
        console.error(error);
      }
      this.searchingEvent = false;
    },
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
