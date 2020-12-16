<template>
  <!--    STEP I-->
  <div class="card-deck mb-4 text-center">
    <div class="card mb-4 shadow-sm">
      <div class="card-header">
        <h4 class="my-0 font-weight-normal"><span><img width="33" height="29" src="/images/ready.webp"></span>  Find your Event</h4>
      </div>
      <div class="card-body">
        <form v-on:submit="findEvent">
          <div class="form-group row">
            <label for="runNameSearch" class="col-sm-2 col-form-label pr-0">Event Name</label>
            <div class="col-sm-8">
              <input v-model.trim="eventName" ref="search" type="text" class="form-control" id="runNameSearch" placeholder="Name of your event (i.e: Marathon Valencia)">
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
    searchEvent: String,
    events: Array,
  },
  data: () => {
    return {
      searchingEvent: false,
      eventName: null,
    }
  },
  emits: ['update:events', 'update:searchEvent'],
  methods: {
    async findEvent(ev: Event) {
      ev.preventDefault();
      if(_.isNil(this.eventName) || _.isEmpty(this.eventName)) {
        this.$refs.search.classList.add("is-invalid");
        return;
      }

      this.searchingEvent = true;
      try {
        // Find events
        const response = await findEventByName(this.eventName)
        this.$emit('update:events', response.data);
        if(ev.type !== 'none') {
          this.$emit('update:searchEvent', this.eventName);
        }
      } catch (error) {
        console.error(error);
      }
      this.searchingEvent = false;
    },
  },
  updated() {
    this.$refs.search.classList.remove("is-invalid");
  },
  watch: {
    searchEvent: {
      immediate: true,
      handler: function (newVal, oldVal) {
        if(_.isNil(oldVal) && !_.isEmpty(newVal)) {
          this.eventName = this.searchEvent;
          this.findEvent(new MouseEvent('none'));
        }
      }
    }
  }
})
export default class SearchEvent extends Vue {
  searchEvent!: string
  events!: Array<any>
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>

.icon-event-type {
  width: 25px;
}


</style>
