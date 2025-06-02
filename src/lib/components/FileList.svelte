<script lang="ts">
  import type { FileInfo } from '../invoke';
  import { FileNavigationState } from './FileNavigationState.svelte';
  import FileRow from './FileRow.svelte';

  interface Props {
    files: FileInfo[];
  }

  let { files }: Props = $props();

  const navigationState = new FileNavigationState();
  let containerElement: HTMLDivElement;

  // Update files when they change
  $effect(() => {
    navigationState.setFiles(files);
  });

  // Handle keyboard events
  function handleKeydown(event: KeyboardEvent) {
    navigationState.handleKeydown(event);
  }

  // Expose focus method for external use
  export function focus() {
    if (containerElement) {
      containerElement.focus();
    }
  }
</script>

<div
  role="list"
  class="file-list"
  bind:this={containerElement}
  tabindex="0"
  onkeydown={handleKeydown}
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
    overflow: hidden;
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
