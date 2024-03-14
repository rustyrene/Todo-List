<script>
    import { onDestroy, onMount } from "svelte";

    export let closeModal;

    onMount(() => {
        const body = document.querySelector("body");
        window.addEventListener("keydown", closeModal)
        body.style.overflow = "hidden";
    });

    onDestroy(() => {
        const body = document.querySelector("body");
        body.style.overflow = "";
    })
</script>

<div class="modal" on:click={closeModal} on:keydown={closeModal} role="button" tabindex="-1">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="box" id="box" on:click={(event) => event.stopPropagation()} role="button" tabindex="0" on:keydown={closeModal}>
        <div class="controls">
            <i></i>
            <h2>Add Category</h2>
            <i class="fa-solid fa-xmark"></i>
        </div>
        <p>Do you want to add a new category?</p>
        <div class="input">
            <label for="category-name">Category:</label>
            <input type="text" id="category-name">
        </div>
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

        background-color: rgba(0, 0, 0, .2);
        z-index: 500;
    }

    .box {
        height: 100px;
        width: 400px;

        display: flex;
        flex-direction: column;
        padding: 10px 30px;
        
        border-radius: 20px;
        background-color: var(--foreground);
        z-index: 1000;
    }

    .box .controls {
        display: flex;
        justify-content: space-between;
    }
</style>