import { invoke } from '@tauri-apps/api/core';

export interface FileInfo {
  name: string;
  file_type: string; // "image", "video", or "pdf"
  created?: number;  // Unix timestamp in seconds
  modified?: number; // Unix timestamp in seconds
}

export async function loadDesktopFiles(searchQuery: string): Promise<FileInfo[]> {
  // // 50/50 chance to throw an error
  // if (Math.random() < 0.5) {
  //   throw new Error('Random error occurred while listing desktop files');
  // }
  return await invoke('list_desktop_files', { searchQuery });
}
