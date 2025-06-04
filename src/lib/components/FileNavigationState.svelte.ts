import { loadDesktopFiles, type FileInfo } from '../invoke';

export class FileNavigationState {
  files = $state<FileInfo[]>([]);
  private _selectedIndex = $state<number>(0);
  private _searchInputRef = $state<HTMLInputElement | null>(null);
  private _fileListRef = $state<HTMLDivElement | null>(null);

  constructor() {
    loadDesktopFiles('').then((files) => {
      this.files = files;
    });
  }

  async search(query: string) {
    this.files = await loadDesktopFiles(query);
  }

  get selectedIndex() {
    return this._selectedIndex;
  }

  get selectedFile() {
    return this.files[this._selectedIndex] || null;
  }

  setSearchInputRef(ref: HTMLInputElement | null) {
    this._searchInputRef = ref;
  }

  setFileListRef(ref: HTMLDivElement | null) {
    this._fileListRef = ref;
  }

  setFiles(files: FileInfo[]) {
    this.files = files;
    this._selectedIndex = files.length > 0 ? 0 : -1;
  }

  focusSearch() {
    this._searchInputRef?.focus();
  }

  focusFileList() {
    this._fileListRef?.focus();
  }

  navigateUp() {
    if (this.files.length === 0) return;

    if (this._selectedIndex === 0) {
      // If on first item, refocus search box
      this.focusSearch();
    } else {
      this._selectedIndex = Math.max(0, this._selectedIndex - 1);
    }
  }

  navigateDown() {
    if (this.files.length === 0) return;
    this._selectedIndex = Math.min(this.files.length - 1, this._selectedIndex + 1);
  }

  openSelected() {
    const file = this.selectedFile;
    if (file) {
      console.log('Opening file:', file.name);
    }
  }

  openFirstItem() {
    if (this.files.length > 0) {
      console.log('Opening file:', this.files[0].name);
    }
  }

  focusSecondItem() {
    if (this.files.length > 1) {
      this._selectedIndex = 1;
    } else if (this.files.length === 1) {
      this._selectedIndex = 0;
    }
    this.focusFileList();
  }

  handleSearchKeydown = (event: KeyboardEvent) => {
    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault();
        this.focusSecondItem();
        break;
      case 'Enter':
        event.preventDefault();
        this.openFirstItem();
        break;
    }
  };

  handleFileListKeydown = (event: KeyboardEvent) => {
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
  };
}
