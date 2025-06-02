<script lang="ts">
  import { loadDesktopFiles } from '../lib/invoke';

  let searchQuery = $state('');
  let files = $derived(await loadDesktopFiles(searchQuery));
</script>

<main class="container">
  <div class="file-browser">
    <input class="search" type="text" bind:value={searchQuery} placeholder="Search" />

    <ul class="file-list">
      {#each files as file}
        <li>{file}</li>
      {/each}
    </ul>
  </div>
</main>

<style>
  .search {
    width: 100%;
    padding: 0.5rem;
    border-radius: 4px;
    font-size: 1.8rem;
    border: none;
    padding-inline: 1rem;
  }
</style>
