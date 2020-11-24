
import { createApp } from 'vue'
import App from './App.vue'

import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'
import feather from 'feather-icons'
import 'datatables.net'
import $ from 'jquery'

createApp(App).mount('#app')

$(document).ready( function () {
    feather.replace();
    $('#events').DataTable();
    $('#trades').DataTable();
} );
