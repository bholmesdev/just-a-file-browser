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

  private scrollToCenter() {
    if (!this._fileListRef) return;

    const container = this._fileListRef;
    
    // Find the currently selected file row element
    const selectedElement = container.children[this._selectedIndex] as HTMLElement;
    if (!selectedElement) return;

    // Get bounding rectangles for both container and selected element
    const containerRect = container.getBoundingClientRect();
    const selectedRect = selectedElement.getBoundingClientRect();
    
    // Calculate the center of the selected element relative to the container
    const selectedElementCenter = selectedRect.top + selectedRect.height / 2;
    
    // Calculate how much we need to scroll to center the element
    const scrollOffset = selectedElementCenter - containerRect.top - containerRect.height / 2;
    
    container.scrollTo({
      top: container.scrollTop + scrollOffset,
      behavior: 'smooth'
    });
  }

  navigateUp() {
    if (this.files.length === 0) return;

    if (this._selectedIndex === 0) {
      // If on first item, refocus search box
      this.focusSearch();
    } else {
      this._selectedIndex = Math.max(0, this._selectedIndex - 1);
      this.scrollToCenter();
    }
  }

  navigateDown() {
    if (this.files.length === 0) return;
    this._selectedIndex = Math.min(this.files.length - 1, this._selectedIndex + 1);
    this.scrollToCenter();
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
