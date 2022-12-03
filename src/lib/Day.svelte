<script lang="ts">
  import { day } from "../store/stores";
  export let dateNum: number;
  export let date: Date;
  export let isToday: boolean;
  let classesToAdd: string;
  export { classesToAdd as class };

  function setDay() {
    console.log(date);
    day.set(date);
  }
  function handleKeypress(e: KeyboardEvent) {
    if (e.key === "Enter") {
      setDay();
    }
  }
</script>

<span
  class={`date ${classesToAdd || ""}`}
  on:click={setDay}
  on:keypress={handleKeypress}
  role="button"
  tabindex="0"
>
  <span class="date__inner" class:today={isToday}>{dateNum}</span>
</span>

<style lang="css">
  .date {
    height: 50px;
    font-size: 16px;
    letter-spacing: -1px;
    border: 1px solid #e6e4e4;
    padding-right: 4px;
    font-weight: 700;
    padding: 0.5rem;
  }

  .date.other {
    opacity: 0.2;
  }
  .date__inner.today {
    display: inline-block;
    width: 25px;
    height: 25px;
    border-radius: 50%;
    background: #5286fa;
    text-align: center;
    color: white;
  }
</style>
