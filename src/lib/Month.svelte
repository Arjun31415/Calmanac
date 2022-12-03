<script>
  import calendarize from "https://unpkg.com/calendarize?module";
  import Arrow from "./Arrow.svelte";
  import Day from "./Day.svelte";

  export let today = new Date(); // Date
  export let year = today.getFullYear(); // number
  export let month = today.getMonth(); // number
  export let offset = 0; // Sun

  export let labels = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
  export let months = [
    "Jan",
    "Feb",
    "Mar",
    "Apr",
    "May",
    "Jun",
    "July",
    "Aug",
    "Sep",
    "Oct",
    "Nov",
    "Dec",
  ];

  $: today_month = today && today.getMonth();
  $: today_year = today && today.getFullYear();
  $: today_day = today && today.getDate();

  let prev = calendarize(new Date(year, month - 1), offset);
  let current = calendarize(new Date(year, month), offset);
  let next = calendarize(new Date(year, month + 1), offset);

  function toPrev() {
    [current, next] = [prev, current];

    if (--month < 0) {
      month = 11;
      year--;
    }

    prev = calendarize(new Date(year, month - 1), offset);
  }

  function toNext() {
    [prev, current] = [current, next];

    if (++month > 11) {
      month = 0;
      year++;
    }

    next = calendarize(new Date(year, month + 1), offset);
  }

  function isToday(day) {
    return (
      today && today_year === year && today_month === month && today_day === day
    );
  }
  console.log(current);
</script>

<header>
  <Arrow left on:click={toPrev} />
  <h4>{months[month]} {year}</h4>
  <Arrow on:click={toNext} />
</header>

<div class="month">
  {#each labels as txt, idx (txt)}
    <span class="label">{labels[(idx + offset) % 7]}</span>
  {/each}

  {#each { length: 6 } as _, idxw (idxw)}
    {#if current[idxw]}
      {#each { length: 7 } as _, idxd (idxd)}
        {#if current[idxw][idxd] != 0}
          <Day
            date={current[idxw][idxd]}
            isToday={isToday(current[idxw][idxd])}
            class="date"
          />
        {:else if idxw < 1}
          <Day
            date={prev[prev.length - 1][idxd]}
            isToday={false}
            class="date other"
          />
        {:else}
          <Day date={next[0][idxd]} isToday={false} class="date other" />
        {/if}
      {/each}
    {/if}
  {/each}
</div>

<style>
  header {
    display: flex;
    margin: 2rem auto;
    align-items: center;
    justify-content: center;
    user-select: none;
  }

  h4 {
    display: block;
    text-align: center;
    text-transform: uppercase;
    font-size: 140%;
    margin: 0 1rem;
  }

  .month {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    text-align: right;
    grid-gap: 4px;
  }

  .label {
    font-weight: 300;
    text-align: center;
    text-transform: uppercase;
    margin-bottom: 0.5rem;
    opacity: 0.6;
  }
</style>
