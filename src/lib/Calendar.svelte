<script lang="ts">
  // @ts-ignore
  import calendarize from "https://unpkg.com/calendarize?module";
  import Arrow from "./Arrow.svelte";
  import Day from "./Day.svelte";

  export let today = new Date(); // Date
  export let year: number = today.getFullYear(); // number
  export let month: number = today.getMonth();
  export let offset: number = 0; // Sun

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
  function jumpToToday() {
    month = today_month;
    year = today_year;
    current = calendarize(new Date(year, month), offset);
  }

  function toNext() {
    [prev, current] = [current, next];

    if (++month > 11) {
      month = 0;
      year++;
    }

    next = calendarize(new Date(year, month + 1), offset);
  }

  function isToday(day: number) {
    return (
      today && today_year === year && today_month === month && today_day === day
    );
  }
</script>

<header>
  <div>
    <Arrow
      left
      on:click={toNext}
      on:keypress={(e) => {
        if (e.key === "Enter") {
          toPrev();
        }
      }}
    />
    <h4>{months[month]} {year}</h4>
    <Arrow
      on:click={toNext}
      on:keypress={(e) => {
        if (e.key === "Enter") {
          toNext();
        }
      }}
    />
  </div>
  <button class="goto_today" tabindex="0" on:click={jumpToToday}>Today</button>
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
            dateNum={current[idxw][idxd]}
            isToday={isToday(current[idxw][idxd])}
            class="date"
            date={new Date(year, month, current[idxw][idxd])}
          />
        {:else if idxw < 1}
          <Day
            dateNum={prev[prev.length - 1][idxd]}
            isToday={false}
            class="date other"
            date={new Date(year, month - 1, prev[prev.length - 1][idxd])}
          />
        {:else}
          <Day
            dateNum={next[0][idxd]}
            isToday={false}
            class="date other"
            date={new Date(year, month + 1, next[0][idxd])}
          />
        {/if}
      {/each}
    {/if}
  {/each}
</div>

<style lang="css">
  header {
    display: flex;
    margin: 2rem auto;
    align-items: center;
    justify-content: center;
    user-select: none;
    flex-direction: column;
  }
  header div {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 1em;
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
  .goto_today {
    background: none;
    border: none;
    color: #000;
    font-size: 1rem;
    font-weight: 300;
    cursor: pointer;
    transition: all 0.3s ease-in-out;
  }
  .goto_today:focus {
    outline: -webkit-focus-ring-color auto 1px;
  }
  .goto_today:hover {
    color: #000;
    background-color: aqua;
    transform: scale(1.1);
  }
</style>
