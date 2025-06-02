<script lang="ts">
  import type { FileInfo } from '../invoke';

  interface Props {
    file: FileInfo;
    isSelected: boolean;
  }

  let { file, isSelected }: Props = $props();

  function formatDate(timestamp?: number): string {
    if (!timestamp) return 'Unknown';
    return new Date(timestamp * 1000).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  }
</script>

<div class="file-row" class:selected={isSelected}>
  <div class="file-name">{file.name}</div>
  <div class="file-modified">{formatDate(file.modified)}</div>
</div>

<style>
  .file-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid #e5e5e5;
    cursor: pointer;
    transition: background-color 0.15s ease;
  }

  .file-row:hover {
    background-color: #f5f5f5;
  }

  .file-row.selected {
    background-color: #e3f2fd;
    border-color: #2196f3;
  }

  .file-name {
    font-weight: 500;
    color: #333;
    flex: 1;
  }

  .file-modified {
    font-size: 0.875rem;
    color: #666;
    white-space: nowrap;
  }
</style>

