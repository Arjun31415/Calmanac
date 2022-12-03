<script lang="ts">
  import Month from "./lib/Month.svelte";
</script>

<main class="container">
  <section>
    <Month month={11} year={2022} />
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
