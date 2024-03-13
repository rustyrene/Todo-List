<script>
  import axios from "axios";
    import { replace } from "svelte-spa-router";

    let username = "test";
    let password = "test";

    const login = async () => {
        const response = await axios.post("http://localhost:3000/user/login", { 
            "username": username, "password":password 
        }, { 
            withCredentials: true
        });
        console.log(response);
    }
</script>

<main>
    <button on:click={login}>Login</button>
</main>