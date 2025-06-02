<script lang="ts">
  import { loadDesktopFiles, type FileInfo } from '../lib/invoke';
  import Icon from '@iconify/svelte';
  import searchLine from '@iconify-icons/mingcute/search-line';
  import FileList from '../lib/components/FileList.svelte';
  import { FileNavigationState } from '../lib/components/FileNavigationState.svelte';

  let searchQuery = $state('');
  let files = $derived(await loadDesktopFiles(searchQuery));
  let searchInput: HTMLInputElement;
  
  const navigationState = new FileNavigationState();
  
  // Update files when they change
  $effect(() => {
    navigationState.setFiles(files);
  });
  
  // Set search input reference and auto-focus
  $effect(() => {
    if (searchInput) {
      navigationState.setSearchInputRef(searchInput);
      searchInput.focus();
    }
  });
</script>

<main class="container">
  <div class="file-browser">
    <div class="search-container">
      <input 
        class="search" 
        type="text" 
        bind:value={searchQuery} 
        bind:this={searchInput}
        placeholder="Search" 
        onkeydown={navigationState.handleSearchKeydown}
      />
      <Icon icon={searchLine} class="search-icon" />
    </div>

    <FileList {navigationState} />
  </div>
</main>

<style>
  .search-container {
    position: relative;
    width: 100%;
  }
  
  .search {
    width: 100%;
    padding: 0.5rem;
    border-radius: 4px;
    font-size: 1.8rem;
    border: none;
    padding-inline: 1rem;
    padding-right: 3rem; /* Make space for the icon */
  }
  
  :global(.search-icon) {
    position: absolute;
    right: 1rem;
    top: 50%;
    transform: translateY(-50%);
    color: #666;
    width: 1.5rem;
    height: 1.5rem;
    pointer-events: none;
  }
</style>
