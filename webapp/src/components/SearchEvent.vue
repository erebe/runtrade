<template>
  <!--    STEP I-->
  <div v-if="displayFindEvent" class="card-deck mb-4 text-center">
    <div class="card mb-4 shadow-sm">
      <div class="card-header">
        <h4 class="my-0 font-weight-normal"><span><img src="/images/ready.png"></span>  Find your Event</h4>
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

</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import _ from 'lodash';
import {findEventByName} from "@/api";

@Options({
  props: {
    eventSearch: String,
    events: Array,
  },
  data: () => {
    return {
      displayFindEvent: true,
      searchingEvent: false,
      eventName: null,
    }
  },
  emits: ['update:events'],
  methods: {
    async findEvent(ev: Event) {
      ev.preventDefault();
      this.searchingEvent = true;
      try {
        // Find events
        const response = await findEventByName(this.eventName)
        //this.events = response.data as Array<any>;
        this.$emit('update:events', response.data);

        // Transition to new state
        this.displaySelectEvent = true;
        if(ev.type !== 'none') {
          window.location.hash = "#findEvent=" + encodeURI(this.eventName);
        }

      } catch (error) {
        console.error(error);
      }
      this.searchingEvent = false;
    },
  },
  mounted() {
    if(!_.isEmpty(this.eventSearch)) {
      this.eventName = this.eventSearch;
      this.findEvent(new MouseEvent('none'));
    }
  }
})
export default class SearchEvent extends Vue {
  eventSearch!: string
  events!: Array<any>
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>

.icon-event-type {
  width: 25px;
}


</style>
