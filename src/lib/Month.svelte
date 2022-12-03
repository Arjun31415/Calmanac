<!-- for a given month draw all the days -->
<script lang="ts">
  import Day from "./Day.svelte";
  export let month: number;
  export let year: number;
  let numDays = new Date(year, month + 1, 0).getDate();
  let days = [];
  for (let i = 1; i <= numDays; i++) {
    days.push(new Date(year, month, i));
  }
  let today = new Date();
</script>

<div class="month">
  <div class="month-name">
    {new Date(2021, month, 1).toLocaleDateString("en-US", { month: "long" })}
  </div>
  <div class="month-days">
    {#each days as day}
      <Day date={day} isToday={day == today} />
    {/each}
  </div>
</div>

<style type="text/css">
  .month {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin: 1rem;
  }
  .month-name {
    font-size: 1.5rem;
    font-weight: bold;
  }
  .month-days {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
  }
</style>
