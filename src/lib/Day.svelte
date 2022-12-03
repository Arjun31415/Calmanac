<!-- Draw a day in the calendar with the given name and date -->
<script lang="ts">
  export let date: Date;
  export let isToday: boolean;
  let dayName = date.toLocaleDateString("en-US", { weekday: "short" });
</script>

<!-- if it is today then highlight the date -->
<div class="day">
  <div class="day-name">{dayName}</div>
  <div class="day-date">{date.getDate()}</div>
</div>
