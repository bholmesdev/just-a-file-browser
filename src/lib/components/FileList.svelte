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

<style>
  .file-list {
    border: 1px solid #ddd;
    border-radius: 6px;
    background: white;
    overflow-y: auto;
    flex: 1;
    min-height: 0;
    outline: none;
  }

  .file-list:focus {
    border-color: #2196f3;
    box-shadow: 0 0 0 2px rgba(33, 150, 243, 0.2);
  }

  .empty-state {
    padding: 2rem;
    text-align: center;
    color: #666;
    font-style: italic;
  }
</style>
