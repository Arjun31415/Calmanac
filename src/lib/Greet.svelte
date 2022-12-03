<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "Arjun is the best";
  let greetMsg = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }
  async function getAllCalendars() {
    let calendars = await invoke("get_all_calendars");
    console.log(calendars);
  }
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={getAllCalendars}> Greet </button>
  </div>
  <p>{greetMsg}</p>
</div>
