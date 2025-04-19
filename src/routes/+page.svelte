<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import SeekBar from "../components/seek_bar.svelte";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<div class="h-full bg-neutral-900 flex flex-col p-1">
  <div class="grid grid-cols-1 gap-4 grow">
    <p class="bg-neutral-700 rounded-2xl row-span-2">e</p>
    <p class="bg-neutral-700 rounded-2xl">f</p>
  </div>
  <div class="pt-4 min-w-full block">
    <SeekBar />
  </div>
</div>
