<script lang="ts">
  import Icon from '@iconify/svelte';
  import searchLine from '@iconify-icons/mingcute/search-line';
  import type { FileNavigationState } from './FileNavigationState.svelte';

  interface Props {
    navigationState: FileNavigationState;
    searchQuery: string;
    searchInputRef: HTMLInputElement | undefined;
    onsearchchange: (query: string) => void;
  }

  let { navigationState, searchQuery, searchInputRef = $bindable(), onsearchchange }: Props = $props();

  function handleInput(event: Event) {
    const target = event.target as HTMLInputElement;
    onsearchchange(target.value);
  }
</script>

<div class="search-container">
  <!-- svelte-ignore a11y_autofocus -->
  <input
    class="search"
    type="text"
    autofocus
    value={searchQuery}
    bind:this={searchInputRef}
    placeholder="Search"
    oninput={handleInput}
    onkeydown={navigationState.handleSearchKeydown}
  />
  <Icon icon={searchLine} class="search-icon" />
</div>

<style>
  .search-container {
    position: relative;
    width: 100%;
    flex-shrink: 0; /* Don't shrink the search box */
    z-index: 1;
  }

  .search {
    width: 100%;
    padding: 0.5rem;
    border-radius: 4px;
    font-size: 1.8rem;
    border: none;
    padding-inline: 1rem;
    padding-right: 3rem; /* Make space for the icon */
    outline: none;
    background: transparent;
  }

  .search-container::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 2px;
    background: repeating-linear-gradient(
      to right,
      #ccc 0px,
      #ccc 4px,
      transparent 4px,
      transparent 8px
    );
    pointer-events: none;
  }

  .search-container:focus-within::after {
    background: repeating-linear-gradient(
      to right,
      #999 0px,
      #999 4px,
      transparent 4px,
      transparent 8px
    );
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

