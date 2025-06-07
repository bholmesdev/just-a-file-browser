<script lang="ts">
  import Icon from '@iconify/svelte';
  import searchLine from '@iconify-icons/mingcute/search-line';
  import FileList from '../lib/components/FileList.svelte';
  import { FileNavigationState } from '../lib/components/FileNavigationState.svelte';

  let searchInput: HTMLInputElement;
  let searchQuery = $state('');

  const navigationState = new FileNavigationState();

  // Set search input reference for navigation
  $effect(() => {
    navigationState.setSearchInputRef(searchInput);
  });

  $effect(() => {
    if (searchQuery) {
      navigationState.search(searchQuery);
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
      <!-- svelte-ignore a11y_autofocus -->
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
  .container {
    height: 100vh;
    padding: 1rem;
  }

  .file-browser {
    display: flex;
    flex-direction: column;
    height: 100%;
    gap: 1rem;
  }

  .search-container {
    position: relative;
    width: 100%;
    flex-shrink: 0; /* Don't shrink the search box */
  }

  .search {
    width: 100%;
    padding: 0.5rem;
    border-radius: 4px;
    font-size: 1.8rem;
    border: none;
    border-bottom: 2px dashed #ccc;
    padding-inline: 1rem;
    padding-right: 3rem; /* Make space for the icon */
    outline: none;
  }

  .search:focus {
    border-bottom-color: #999;
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
