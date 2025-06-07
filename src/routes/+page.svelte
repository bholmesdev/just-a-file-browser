<script lang="ts">
  import SearchInput from '../lib/components/SearchInput.svelte';
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

  function handleSearchChange(newQuery: string) {
    searchQuery = newQuery;
  }

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
    <SearchInput 
      {navigationState} 
      {searchQuery} 
      bind:searchInputRef={searchInput}
      onsearchchange={handleSearchChange} 
    />
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

</style>
