<template>
  <div class="modal fade" id="addTrade" tabindex="-1" role="dialog" aria-labelledby="AddTradeLabel" aria-hidden="true">
    <div class="modal-dialog modal-xl">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title" id="exampleModalLongTitle">Add your trade</h5>
          <button type="button" class="close" data-dismiss="modal" aria-label="Close">
            <span aria-hidden="true">&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <form>
            <!--            EVENT NAME-->
            <div class="form-group row">
              <label for="eventName" class="col-sm-2 col-form-label">Event Name</label>
              <div class="col-sm-10">
                <input ref="name" v-model.trim="this.event.name" type="text" class="form-control" id="eventName" readonly disabled>
              </div>
            </div>

            <!--            EVENT DISTANCE-->
            <div class="form-group row">
              <label for="eventCategory" class="col-sm-2 col-form-label">Category</label>
              <div class="col-sm-10">
                <input ref="category" v-model.trim="this.trade.category" type="text" class="form-control" id="eventCategory"
                       placeholder="Category/Distance of the event if any (i.e: 10km, 21km, 42km, ...)" >
                <div class="invalid-feedback text-left">
                  Cannot be empty
                </div>
              </div>
            </div>

            <div class="form-group row">
              <!--            USER NAME-->
              <label for="user" class="col-sm-2 col-form-label">User</label>
              <div class="col-sm-6">
                <input ref="name" v-model.trim="this.user.name" type="text" class="form-control" id="user" readonly disabled>
              </div>

              <!--            USER GENDER-->
              <label for="gender" class="col-sm-1 col-form-label">Gender</label>
              <div class="col-sm-3">
                <select ref="gender" v-model="this.trade.gender" class="form-control" id="gender" >
                  <option value="Man">Man</option>
                  <option value="Woman">Woman</option>
                </select>
              </div>
            </div>

            <div class="form-group row">
              <!--            TRADE INTENT-->
              <label for="intent" class="col-sm-2 col-form-label">Intent</label>
              <div class="col-sm-6">
                <select ref="intent" v-model="this.trade.intent" class="form-control" id="intent" >
                  <option value="Buy">Buy: You want to buy an inscription to this event</option>
                  <option value="Sell">Sell: You want to sell your inscription to this event</option>
                </select>
              </div>

              <!--              TRADE PRICE-->
              <label for="price" class="col-sm-1 col-form-label">Price</label>
              <div class="col-sm-3">
                <input ref="price" v-model.trim="this.price" type="text" class="form-control" id="price"
                       placeholder="24€" min="0">
                <div class="invalid-feedback text-left">
                  Price should contain currency (i.e 12.5€)
                </div>
              </div>

            </div>

            <!--            USER CONTACT-->
            <div class="form-group row">
              <label for="Contact" class="col-sm-2 col-form-label">Contact</label>
              <div class="col-sm-10">
                <input ref="contact" v-model.trim="this.contact" type="text" class="form-control" id="contact"
                       placeholder="How to contact you (i.e: email or Facebook messenger https://m.me/erebe.dellu.42)" >
                <div class="invalid-feedback text-left">
                  You need to provide a way to contact you
                </div>
              </div>
            </div>

            <div class="form-group row">
              <label for="note" class="col-sm-2 col-form-label">Note</label>
              <div class="col-sm-10">
                <textarea ref="note" v-model.trim="this.trade.note" class="form-control" id="note"
                          placeholder="Some information you want other to know about your trade" >
                </textarea>
              </div>
            </div>
          </form>

          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" data-dismiss="modal">Close</button>
            <button type="submit" @click="this.submit" class="btn btn-primary">Add your trade</button>
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
import * as Api from "@/api"
import _ from 'lodash';
import {getAppContext} from "@/main";
import { PropType } from 'vue';

@Options({
  emits: ['close'],
  props: {
    event: Object as PropType<Api.Event>
  },
  data: () => {
    const user = getAppContext().user!;
    return {
      user: user,
      price: "",
      contact: user.contact,
      trade: null as unknown as Api.Inscription
    }
  },
  methods: {
    async submit(ev: MouseEvent) {
      ev.preventDefault();
      let isValid = true;

      const trade: Api.Inscription = this.trade;

      if(_.isEmpty(this.price) || this.$refs.price.classList.contains("is-invalid")) {
        this.$refs.price.classList.remove("is-valid");
        this.$refs.price.classList.add("is-invalid");
        isValid = false;
      }

      if(_.isEmpty(this.contact) || this.$refs.contact.classList.contains("is-invalid")) {
        this.$refs.contact.classList.remove("is-valid");
        this.$refs.contact.classList.add("is-invalid");
        isValid = false;
      }

      if(!isValid) {
        return;
      }

      trade.price = parseFloat(this.price.slice(0, -1));
      trade.currency = this.price.slice(-1);
      if(this.contact !== this.user.contact) {
        this.user.contact = this.contact;
        await Api.updateUserContact(this.user, this.contact);
      }
      const inscription = (await Api.addTrade(trade)).data;
      window.location.reload();
    }
  },
  async created() {
    this.trade = Api.newInscription(getAppContext().user!, this.event)
  },
  mounted() {
    const el = $('#addTrade');
    const modal = new Modal(el[0]);
    el.on('hidden.bs.modal', () => this.$emit('close'));
    modal.show();
  },
  updated() {
    const trade: Api.Inscription = this.trade;
    if (!_.isEmpty(this.price)) {
      if(this.price.match(/^[0-9]+(\.[0-9]{1,2})?[^.\-_0-9]$/)) {
        this.$refs.price.classList.remove("is-invalid");
        this.$refs.price.classList.add("is-valid");
      } else {
        this.$refs.price.classList.remove("is-valid");
        this.$refs.price.classList.add("is-invalid");
      }
    }

    if (!_.isEmpty(this.trade.category)) {
      this.$refs.category.classList.remove("is-invalid");
      this.$refs.category.classList.add("is-valid");
    }

    if (!_.isEmpty(this.contact)) {
        this.$refs.contact.classList.remove("is-invalid");
        this.$refs.contact.classList.add("is-valid");
    } else {
      this.$refs.contact.classList.remove("is-valid");
    }
  }
})
export default class AddTrade extends Vue {
  event!: Api.Event
}
</script>




<style scoped>
</style>
