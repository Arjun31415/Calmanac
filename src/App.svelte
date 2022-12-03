<script lang="ts">
  import Calendar from "./lib/Calendar.svelte";
</script>

<main class="container">
  <section>
    <Calendar month={11} year={1977} />
  </section>
</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
