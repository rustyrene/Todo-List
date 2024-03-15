<script>
  import axios from "axios";
    import { onDestroy, onMount } from "svelte";

    export let closeModal;

    let category_name = "";

    const addCategory = async () => {
        try {
            const response = await axios.post("http://localhost:3000/category/add-category", {
            "category_name": category_name }, {
                    withCredentials: true
            });
        } catch(err) {
            console.log(err);
        }
        category_name = "";
        closeModal();
    }

    const handleKeyEvent = (event) => {
        if (event.key === "Escape") {
            closeModal();
        }
    }

    onMount(() => {
        const body = document.querySelector("body");
        body.style.overflow = "hidden";
    });

    onDestroy(() => {
        const body = document.querySelector("body");
        body.style.overflow = "";
    })
</script>

<div class="modal" on:click={closeModal} on:keydown={(event) => handleKeyEvent(event)} role="button" tabindex="-1">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="box" id="box" on:click={(event) => event.stopPropagation()} role="button" tabindex="0" on:keydown={(event) => handleKeyEvent(event)}>
        <div class="controls">
            <i></i>
            <h2>Add Category</h2>
            <button on:click={closeModal}><i class="fa-solid fa-xmark"></i></button>
        </div>
        <p>Do you want to add a new category?</p>
        <div class="input">
            <label for="category-name">Category:</label>
            <input type="text" id="category-name" bind:value={category_name}>
        </div>
        <button class="submit" on:click={addCategory}>Add</button>
    </div>
</div>

<style>
    .modal {
        position: absolute;
        display: flex;
        justify-content: center;
        align-items: center;
        left: 0;
        top: 0;
        margin-top: 60px;

        height: 100vh;
        width: 100%;

        background-color: rgba(0, 0, 0, .3);
        z-index: 500;
    }

    .box {
        height: 160px;
        width: 400px;

        display: flex;
        flex-direction: column;
        padding: 14px 30px;
        padding-bottom: 30px;
        gap: 10px;
        
        border-radius: 20px;
        background-color: var(--foreground);
        z-index: 1000;
    }

    .box .controls {
        display: flex;
        justify-content: space-between;
        padding-bottom: 10px;
    }

    .box .controls {
        font-size: 16px;
    }

    .box .controls button {
        background: transparent;
        border: none;

        font-size: 20px;
        text-align: center;
        color: rgba(0, 0, 0, .6);

        cursor: pointer;
    }

    .box .controls button:hover {
        color: rgba(0, 0, 0, .9);
    }

    .box p {
        font-size: 14px;
    }

    .box .input {
        display: flex;
        gap: 4px;
        flex-direction: column;
    }

    .box .input input {
        height: 26px;
        background-color: rgba(0, 0, 0, .1);
    }

    .box button.submit {
        padding: 4px;

        font-size: 18px;
        text-align: center;
        border: none;
        color: rgba(255, 255, 255, .8);
        font-weight: 600;

        background-color: var(--primary);
        cursor: pointer;
    }

    .box button.submit:hover {
        color: white;
        background-color: var(--primary-dark);
    }
</style>