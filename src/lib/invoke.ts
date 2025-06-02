import { invoke } from '@tauri-apps/api/core';

export async function loadDesktopFiles(searchQuery: string): Promise<string[]> {
  // // 50/50 chance to throw an error
  // if (Math.random() < 0.5) {
  //   throw new Error('Random error occurred while listing desktop files');
  // }
  return await invoke('list_desktop_files', { searchQuery });
}
