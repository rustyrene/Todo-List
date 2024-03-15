<script>
    import axios from "axios";
    import { pop, replace } from "svelte-spa-router";
    import CategoryModal from "./category-modal.svelte";
  import { onMount } from "svelte";

    let popUp = true;
    let categories = [];

    const reload = () => {
        window.location.reload();
    }

    const logout = async () => {
        try {
            await axios.get("http://localhost:3000/user/logout", {
                withCredentials:true
            });
            replace("/login")
        } catch(err) {
            replace("/login")
        }
    } 

    const modal = () => {
        popUp = !popUp;
    }

    onMount(() => {
        console.log("");
    })
</script>

{#if popUp}
    <CategoryModal closeModal={modal} />
{/if}
<main>
    <nav>
        <button></button>
        <button class="headline" on:click={reload}>Task IT</button>
        <button class="logout" on:click={logout}>Logout</button>
    </nav>
    <div class="wrapper">
        <div class="sidebar">
            <ul>
                <li><button>Overview</button></li>
                <li><button on:click={() => {popUp=true}}><i class="fa-solid fa-circle-plus"></i></button></li>
            </ul>
        </div>
        <div class="content">
        </div>
    </div>
</main> 

<style>
    main {
        min-height: 100vh;
    }

    nav {
        width: 100%;
        height: 60px;

        display: flex;
        position: fixed;
        top: 0;
        justify-content: space-between;
        align-items: center;

        margin-top: 0 !important;

        background-color: var(--primary);
        box-shadow: 0 5px 5px rgba(0, 0, 0, .2);
    }

    nav button {
        border: none;
        background-color: transparent;
        cursor: pointer;
    }

    nav button.headline {
        font-size: 36px;
        font-weight: 600;
        color: rgba(255, 255, 255, 1);
    }

    nav button.logout {
        margin-right: 46px;
        font-size: 26px;
        color: rgba(255, 255, 255, .8);
        transition: .1s ease-out;
    }

    nav button.logout:hover {
        color: white;
    }

    .wrapper {
        display: flex;
        min-height: 100vh;
        width: 100%;
        margin-top: 60px;
    }

    .sidebar {
        width: 300px;
        height: 100%;
        position: fixed;

        background-color: var(--primary-dark);
        box-shadow: 3px 0 5px rgba(0, 0, 0, .2);

        z-index: 50;
    }

    .sidebar ul {
        margin-top: 36px;
    }

    .sidebar ul li {
        width: 100%;
        height: 54px;
        margin-top: 4px;
    }

    .sidebar ul li button {
        width: 100%;
        padding: 10px 0;

        text-decoration: none;
        font-size: 26px;
        color: rgba(255, 255, 255, .8);
        text-align: center;

        background-color: transparent;
        border: none;
        
        transition: .1s ease-out;
        cursor: pointer;
    }

    .sidebar ul li button:hover {
        background-color: var(--primary-light);
    }

    .sidebar ul li button i {
        font-size: 22px;
    }

    .content {
        flex: 1;
        position: fixed;
        margin-left: 300px;
        background-color: var(--background);
    }
</style>