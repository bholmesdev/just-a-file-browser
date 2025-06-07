<script lang="ts">
  import type { FileInfo } from '../invoke';
  import Icon from '@iconify/svelte';
  import fileGeneral from '@iconify-icons/mingcute/file-line';
  import folderLine from '@iconify-icons/mingcute/folder-line';
  import imageLine from '@iconify-icons/mingcute/pic-line';

  interface Props {
    file: FileInfo;
    isSelected: boolean;
  }

  let { file, isSelected }: Props = $props();

  function formatDate(timestamp?: number): string {
    if (!timestamp) return 'Unknown';
    const date = new Date(timestamp * 1000);
    const now = new Date();
    const diffTime = now.getTime() - date.getTime();
    const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));
    
    if (diffDays === 0) {
      return 'Today';
    } else if (diffDays === 1) {
      return 'Yesterday';
    } else {
      return date.toLocaleDateString('en-US', {
        month: 'short',
        day: 'numeric',
        year: 'numeric'
      });
    }
  }

  function getFileIcon(fileName: string, isDirectory: boolean) {
    if (isDirectory) {
      return folderLine;
    }
    
    const ext = fileName.split('.').pop()?.toLowerCase();
    if (['jpg', 'jpeg', 'png', 'gif', 'svg', 'webp'].includes(ext || '')) {
      return imageLine;
    }
    
    return fileGeneral;
  }
</script>

<div class="file-row" class:selected={isSelected}>
  <div class="file-name">
    <Icon icon={getFileIcon(file.name, file.is_directory)} class="file-icon" />
    <span class="file-text">{file.name}</span>
  </div>
  <div class="file-modified">{formatDate(file.modified)}</div>
  <div class="file-location">
    <Icon icon={folderLine} class="location-icon" />
    <span class="location-text">Documents</span>
  </div>
</div>

<style>
  .file-row {
    display: grid;
    grid-template-columns: 1fr auto auto;
    align-items: center;
    padding: 0.75rem 2rem;
    cursor: pointer;
    outline: none;
    transition: background-color 0.15s ease;
  }

  .file-row:hover {
    background-color: #f8f9fa;
  }

  .file-row.selected {
    background-color: #ff9500;
    color: white;
  }

  .file-row.selected .file-modified,
  .file-row.selected .location-text {
    color: rgba(255, 255, 255, 0.8);
  }

  .file-row.selected :global(.file-icon),
  .file-row.selected :global(.location-icon) {
    color: white;
  }

  .file-name {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-weight: 500;
    color: #212529;
  }

  :global(.file-icon) {
    width: 1.25rem;
    height: 1.25rem;
    color: #6c757d;
    flex-shrink: 0;
  }

  .file-text {
    flex: 1;
    min-width: 0;
  }

  .file-modified {
    font-size: 0.875rem;
    color: #6c757d;
    white-space: nowrap;
    margin-right: 2rem;
  }

  .file-location {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    color: #6c757d;
    white-space: nowrap;
  }

  :global(.location-icon) {
    width: 1rem;
    height: 1rem;
    color: #6c757d;
  }

  .location-text {
    color: #6c757d;
  }
</style>

