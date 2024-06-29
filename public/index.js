const { createApp, ref } = Vue;
const { createVuetify } = Vuetify;

const vuetify = createVuetify({
  theme: {
    defaultTheme: "dark"
  }
});

const app = createApp({
setup() {
  const title = ref("Hello Brazil!");
  const message = ref("");
  const messages = ref([]);

  const es = new EventSource("/sse");

  es.onopen = () => {
    console.log("Connection opened");
  }

  es.onmessage = (e) => {
    console.log(e.data);
    messages.value = [...messages.value, e.data];
  }

  const handleSubmit = () => {
    fetch("/messages", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        message: message.value
      })
    });

    message.value = "";
  }

  return {
    title,
    message,
    messages,
    handleSubmit
  }
}
});

app
  .use(vuetify)
  .component("my-card", MyCard)
  .mount("#app");