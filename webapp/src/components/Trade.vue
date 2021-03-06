<template>
  <!--    STEP III-->
  <div id="tradesContainer" class="card-deck mb-4 text-center">
    <div class="card mb-4 shadow-sm">

      <div class="card-header d-sm-inline-flex justify-content-sm-between">
        <span class="col-2 d-none d-md-inline-flex"></span>
        <span>
          <h4 class="my-sm-0 font-weight-normal">
            <span><img width="28" height="35" src="/images/go.webp"></span>  Trade
          </h4>
        </span>
        <span class="mx-0 px-0">
<!--          <a class="d-inline-flex mr-3"><span data-feather="link"></span>  Link</a>-->
          <button class="btn btn btn-block btn-primary mx-0" data-container="body"
                  data-content="You want to trade your inscription ? Or propose to buy one ?<br/> Click and add yours !" data-html="true" data-placement="right"
                  type="button"
                  v-on:click="createTrade"
          >
            <span data-feather="plus"></span> Add your Trade
          </button>

          <!--          Modal for adding event-->
          <AddTrade v-if="displayAddTrade" :event="this.event" @close="displayAddTrade = false"></AddTrade>
        </span>
      </div>

      <div class="card-body table-responsive-sm">
        <h5>{{ capitalize(event.name) }}</h5>
        <table id="trades" class="table table-sm table-hover text-left"
               data-order='[[ 1, "desc" ], [4, "desc"], [3, "asc"], [6, "asc"]]'>
          <thead>
          <tr>
            <th scope="col">Contact</th>
            <th scope="col">Intent</th>
            <th scope="col">Event</th>
            <th scope="col">Category</th>
            <th scope="col">Gender</th>
            <th scope="col">Type</th>
            <th scope="col">Price</th>
            <th scope="col">Note</th>
          </tr>
          </thead>
          <tbody>
          <tr v-for="[inscription, user, event] in inscriptions" :key="inscription.id"
              :class="(inscription.intent === 'Buy') ? 'table-primary': 'table-success'"
              :data-content="this.getContact(user)" data-container="body" data-html="true" data-placement="auto"
              data-toggle="popover"
          >
            <td>
              <span v-if="(user.id === this.user?.id)" @click="this.deleteInscription(inscription)" title="delete your trade">
                <span data-feather="trash-2" color="red" ></span>
                {{ user.name }}
              </span>
              <span v-else>
              <a :href="(user.contact.indexOf('@') >= 0) ? 'mailto:' + user.contact : user.contact" target="_blank" rel="noopener noreferrer">
                <span data-feather="send"></span>
                {{ user.name }}
              </a>
              </span>
            </td>
            <td>
              <i :data-feather="(inscription.intent === 'Buy') ? 'crosshair': 'dollar-sign'"></i> {{
                inscription.intent
              }}
            </td>
            <td>{{ event.name }}</td>
            <td>{{ inscription.category }}</td>
            <td>{{ inscription.gender }}</td>
            <td>{{ event.event_type }}</td>
            <td class="text-left">{{ inscription.price + ' ' + inscription.currency }}</td>
            <td class="text-justify" style="width: 150px">{{ inscription.note }}</td>
          </tr>

          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import {Options, Vue} from 'vue-class-component';
import AddTrade from "@/components/AddTrade.vue";
import $ from "jquery";
import 'datatables.net'
import 'datatables.net-bs4/js/dataTables.bootstrap4.min'
import 'datatables.net-bs4/css/dataTables.bootstrap4.min.css'
import feather from 'feather-icons'
import {Popover} from "bootstrap";
import * as Api from '@/api';
import _ from "lodash";
import {Inscription, User} from '@/api';
import {PropType} from 'vue';
import {getAppContext} from "@/main";

@Options({
  components: {
    AddTrade,
  },
  props: {
    user: Object as PropType<Api.User>,
    event: Object as PropType<Api.Event>,
    inscriptions: Array as PropType<Array<[Api.Inscription, Api.User, Api.Event]>>
  },
  data: () => {
    return {
      datatable: null,
      displayAddTrade: false,
    }
  },
  emits: ['update:inscriptions'],
  methods: {
    capitalize(str: string): string {
      return _.capitalize(str);
    },
    init(): void {
      this.datatable = $('#trades').DataTable({pageLength: 50, deferRender: true});
      $('#tradesContainer [data-toggle="popover"]').get().forEach((el) => {
        (el as any).popover = new Popover(el, {trigger: "hover", delay: {show: 0, hide: 1000}});
      });
      feather.replace();
      document.getElementById("tradesContainer")?.scrollIntoView({behavior: "smooth"});
    },
    getContact(user: User): string {
      const contact = (user.contact.indexOf("@") >= 0)
          ? 'mailto:' + user.contact
          : user.contact;
      return _.capitalize(user.name) +
          " wish to be contacted via<br/><a target='_blank' rel='noopener noreferrer' href='" + contact + "'>" +
          user.contact + "</a>"
    },
    async deleteInscription(ev: Inscription) {
      await Api.deleteInscription(ev.id!);
      $('#tradesContainer [data-toggle="popover"]').get().forEach((el) => {
        (el as any).popover.dispose();
      });
      $(".popover").remove();

      _.remove(this.inscriptions, (inscription: [Inscription]) => inscription[0].id === ev.id)
      this.$emit('update:inscriptions', this.inscriptions);

      // window.location.reload();
    },
    createTrade(ev: MouseEvent) {
      ev.preventDefault();
      const keycloak = getAppContext().keycloak;
      if (!keycloak.authenticated || _.isNil(getAppContext().user)) {
        keycloak.login();
      }
      this.displayAddTrade = true;
    }
  },
  mounted() {
    this.init();
  },

  beforeUpdate() {
    this.datatable.destroy();
    this.datatable = null;
  },
  updated() {
    this.init();
  },
})
export default class Trade extends Vue {
  user?: Api.User
  event!: Api.Event
  inscriptions!: Array<[Api.Inscription, Api.User, Api.Event]>
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>

.card-header h4 {
  width: 100%;
}
.feather-trash-2, .feather-send {
  width: 20px;
  height: 20px;
}
</style>
