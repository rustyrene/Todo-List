<script>
  import { onMount } from "svelte";
  import Router from "svelte-spa-router";
  import { push } from "svelte-spa-router";
  import Login from "./Login.svelte";
  import Main from "./Main/Main.svelte";
  import axios from "axios";

  const routes = {
    '/login': Login,
    '/home': Main,
  }

  //!TODO Check if user has valid cookie and redirect accordingly
  onMount(async () => {
    //Check if token is still valid
    try {
      const response = await axios.get("http://localhost:3000/token-check", {withCredentials:true});
      if (response.status === 200) {
        push("/home");
      } else {
        push("/login")
      }
    } catch(err) {
      push("/login")
    }
  })
</script>

<Router {routes} />
