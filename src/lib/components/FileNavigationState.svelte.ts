import type { FileInfo } from '../invoke';

export class FileNavigationState {
  private _files = $state<FileInfo[]>([]);
  private _selectedIndex = $state<number>(0);

  constructor() {}

  get files() {
    return this._files;
  }

  get selectedIndex() {
    return this._selectedIndex;
  }

  get selectedFile() {
    return this._files[this._selectedIndex] || null;
  }

  setFiles(files: FileInfo[]) {
    this._files = files;
    this._selectedIndex = files.length > 0 ? 0 : -1;
  }

  navigateUp() {
    if (this._files.length === 0) return;
    this._selectedIndex = Math.max(0, this._selectedIndex - 1);
  }

  navigateDown() {
    if (this._files.length === 0) return;
    this._selectedIndex = Math.min(this._files.length - 1, this._selectedIndex + 1);
  }

  openSelected() {
    const file = this.selectedFile;
    if (file) {
      console.log('Opening file:', file.name);
    }
  }

  handleKeydown(event: KeyboardEvent) {
    switch (event.key) {
      case 'ArrowUp':
        event.preventDefault();
        this.navigateUp();
        break;
      case 'ArrowDown':
        event.preventDefault();
        this.navigateDown();
        break;
      case 'Enter':
        event.preventDefault();
        this.openSelected();
        break;
    }
  }
}
