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
  const tasks = Vue.ref([]);

  const fetchTasks = async () => {
    /**
     * @method 
     *  create
     *  read
     *  list
     *  update
     *  delete
     *  archive
     */
    const [result, error] = await rpc.call("task.archive", { taskId: 23 });

    if (error) {
      console.log(error);
      return;
    }

    tasks.value = result;
  }

  Vue.onMounted(async () => {
    await Promise.all([
      fetchTasks(),
    ])
    console.log("MyCard component mounted");
  });

  return {
    timestamp
  }
}
})