<script lang="ts">
  import { loadDesktopFiles } from '../lib/invoke';
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

  $effect(() => {
    // Refocus input whenever the query updates
    if (searchQuery) {
      searchInput?.focus();
    }
  });

  function handleSearchWhenInputIsBlurred(event: KeyboardEvent) {
    if (document.activeElement === searchInput || !isKeyboardEventWithoutModifiers(event)) {
      return;
    }
    event.preventDefault();
    if (event.key === 'Backspace' || event.key === 'Delete') {
      searchQuery = searchQuery.slice(0, -1);
    } else {
      searchQuery += event.key;
    }
  }

  function isKeyboardEventWithoutModifiers(event: KeyboardEvent): boolean {
    const isBackspace = event.key === 'Backspace' || event.key === 'Delete';
    return (
      isBackspace || (event.key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey)
    );
  }
</script>

<svelte:window on:keydown={handleSearchWhenInputIsBlurred} />

<main class="container">
  <div class="file-browser">
    <div class="search-container">
      <input
        class="search"
        type="text"
        autofocus
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
