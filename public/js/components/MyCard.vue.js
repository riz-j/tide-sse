const MyCard = Vue.defineComponent({
template: `
    <v-card class="message-card d-flex justify-space-between align-center">
      <v-card-text class="message-text">{{ message }}</v-card-text>
      <v-spacer></v-spacer>
      <v-card-text class="timestamp-text text-end">{{ timestamp }}</v-card-text>
    </v-card>
`,
props: {
  message: {
    type: String,
    default: ""
  }
},
setup() {
  const timestamp = moment().format("hh:mm:ss A");


  Vue.onMounted(() => console.log("MyCard component mounted"))

  return {
    timestamp
  }
}
})