<script lang="ts">
    import { onMount } from "svelte";
    import { day } from "../store/stores";
    $: currentDate = null;
    export let hidden;
    $: day.subscribe((value) => {
        currentDate = value;
    });
</script>

<!-- hshowidden a floating window -->
<div class="day-details" {hidden}>
    <div class="day-details__inner">
        <div class="day-details__header">
            <h3>{currentDate.toDateString()}</h3>
            <button on:click={() => (hidden = true)}> X </button>
        </div>
        <div class="day-details__body">
            <p>Some text</p>
        </div>
    </div>
</div>

<style>
    .day-details {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.5);
        z-index: 100;
    }
    .day-details__inner {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 400px;
        height: 300px;
        background-color: white;
        border-radius: 5px;
        padding: 1rem;
    }
    .day-details__header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .day-details__body {
        margin-top: 1rem;
    }
</style>
