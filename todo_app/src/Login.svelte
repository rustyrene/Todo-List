<script>
  import axios from "axios";
    import { replace } from "svelte-spa-router";

    let username = "";
    let password = "";

    let loading = false;

    const login = async (event) => {
        event.preventDefault();
        loading = true;
        try {
            const response = await axios.post("http://localhost:3000/user/login", { 
                "username": username, "password":password 
            }, { 
                withCredentials: true
            });
            if (response.status === 200) {
                loading = false;
                replace("/home");
            } else {
                password = "";
            }
        } catch(err) {
            setTimeout(() => {
                password = "";
                loading = false;
            }, 600);
        }
    }
</script>

<main>
    <div class="login" class:animate={loading}>
        <h1>Task IT</h1>
        <form>
            <div>
                <label for="username">Username:</label>
                <input type="text" id="username" bind:value={username}>
            </div>
            <div>
                <label for="password">Password:</label>
                <input type="password" id="password" bind:value={password}>
            </div>
            <button class="submit" on:click={(event) => login(event)}>Login</button>
        </form>
    </div>
</main>

<style>
    @keyframes loading {
        from {
            transform: translateX(-100%);
        }
        to {
            transform: translate(0%);
        }
    }

    main {
        display: flex;
        justify-content: center;
        align-items: center;
        position: relative;

        height: 100vh;
    }

    .login {
        display: flex;
        flex-direction: column;
        gap: 10px;
        position: relative;

        padding: 12px 20px;
        padding-bottom: 30px;

        background-color: var(--foreground);
        border-radius: 6px;

        box-shadow: 4px 4px 10px rgba(0, 0, 0, .2);
        overflow: hidden;
    }

    .animate::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 4px; 
        background: linear-gradient(to right, var(--primary-light), var(--primary-dark)); 
        animation: loading 2s forwards; 
        animation-timing-function: cubic-bezier(0.075, 0.82, 0.165, 1);
    }

    .login h1 {
        display: inline-block;
        width: 170px;

        font-size: 36px;
        color: transparent;

        background: linear-gradient(180deg, var(--primary-light), var(--primary-dark));
        background-clip: text;
    }

    form {
        display: flex;
        flex-direction: column;

        gap: 30px;

        width: 500px;
    }

    form div {
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    form .submit {
        align-self: center;
    }

    form button {
        padding: 4px 36px;

        font-size: 28px;
        color: var(--primary-content);

        border: none;
        border-radius: 6px;

        transition: ease-out .05s;

        background-color: var(--primary);
        cursor: pointer;
    }

    form button:hover {
        background-color: var(--primary-dark);
    }

    form input {
        height: 38px;
        font-size: 18px;
        border: none;
        border-radius: 6px;
        padding-left:  20px;
        background-color: rgba(0, 0, 0, .1);
    }

    form label {
        color: var(--copy);
        opacity: .7;
        font-size: 18px;
    }
</style>