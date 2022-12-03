<script lang="ts">
  export let date: number;
  export let isToday: boolean;
  let classesToAdd;
  export { classesToAdd as class };
</script>

<span class={`date ${classesToAdd || ""}`}>
  <span class="date__inner" class:today={isToday}>{date}</span>
</span>

<style type="text/css">
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
