<script lang="ts">
  import { loadDesktopFiles, type FileInfo } from '../lib/invoke';

  let searchQuery = $state('');
  let files = $derived(await loadDesktopFiles(searchQuery));
  
  function formatDate(timestamp?: number): string {
    if (!timestamp) return 'Unknown';
    return new Date(timestamp * 1000).toLocaleString();
  }
</script>

<main class="container">
  <div class="file-browser">
    <input class="search" type="text" bind:value={searchQuery} placeholder="Search" />

    <ul class="file-list">
      {#each files as file}
        <li class="file-item">
          <div class="file-name">{file.name}</div>
          <div class="file-dates">
            <span class="file-date">Created: {formatDate(file.created)}</span>
            <span class="file-date">Modified: {formatDate(file.modified)}</span>
          </div>
        </li>
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
