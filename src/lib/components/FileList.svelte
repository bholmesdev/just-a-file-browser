<script lang="ts">
  import type { FileNavigationState } from './FileNavigationState.svelte';
  import FileRow from './FileRow.svelte';

  interface Props {
    navigationState: FileNavigationState;
  }

  let { navigationState }: Props = $props();

  let containerElement: HTMLDivElement;

  // Set the file list reference when component mounts
  $effect(() => {
    navigationState.setFileListRef(containerElement);
  });
</script>

<div class="file-browser-container">
  <!-- Column headers -->
  <div class="column-headers">
    <div class="header-name">Name</div>
    <div class="header-modified">Modified</div>
    <div class="header-found-in">Found in</div>
  </div>

  <!-- File list -->
  <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="file-list"
    bind:this={containerElement}
    tabindex="0"
    onkeydown={navigationState.handleFileListKeydown}
  >
    {#each navigationState.files as file, index}
      <FileRow {file} isSelected={index === navigationState.selectedIndex} />
    {/each}

    {#if navigationState.files.length === 0}
      <div class="empty-state">No files found</div>
    {/if}
  </div>
</div>

<style>
  .file-browser-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: white;
    border-radius: 12px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    overflow: hidden;
  }

  .column-headers {
    display: grid;
    grid-template-columns: 1fr auto auto;
    align-items: center;
    padding: 0.75rem 2rem;
    background: #f8f9fa;
    border-bottom: 1px solid #e9ecef;
    font-size: 0.875rem;
    font-weight: 500;
    color: #6c757d;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .header-name {
    flex: 1;
  }

  .header-modified {
    text-align: center;
    margin-right: 2rem;
  }

  .header-found-in {
    text-align: right;
  }

  .file-list {
    flex: 1;
    overflow-y: auto;
    outline: none;
  }

  .empty-state {
    padding: 3rem 2rem;
    text-align: center;
    color: #6c757d;
    font-style: italic;
  }
</style>
