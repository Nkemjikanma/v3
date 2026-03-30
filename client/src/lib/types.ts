// ============ API Response Wrapper ============

export type APIResponse<T> = {
  response_message: string;
  response_data: T;
};

// ============ Books ============

export type BookStatus = "reading" | "finished";
export type BookCategory = "technical" | "leisure" | "music";

export type Book = {
  id: string;
  title: string;
  author: string;
  status: BookStatus;
  category: BookCategory;
  year_read: number;
  created_at: string;
  updated_at: string;
};

export type BookFormData = {
  title: string;
  author: string;
  status: BookStatus;
  category: BookCategory;
};

export type UpdateBookFormData = {
  title?: string;
  author?: string;
  status?: BookStatus;
  category?: BookCategory;
};

export type BookQueryInfo = {
  category?: BookCategory;
  status?: BookStatus;
  year_read?: number;
};

// ============ Songs ============

export type Instrument = "guitar" | "piano" | "both";

export type Song = {
  id: string;
  title: string;
  artist: string;
  instrument: Instrument;
  started_learning_at: string; // "YYYY-MM-DD"
  notes: string | null;
  created_at: string;
  updated_at: string;
};

export type SongFormData = {
  title: string;
  artist: string;
  instrument: Instrument;
  started_learning_at: string; // "YYYY-MM-DD"
  notes?: string;
};

export type UpdateSongFormData = {
  title?: string;
  artist?: string;
  instrument?: Instrument;
  notes?: string;
  started_learning_at?: string;
};

export type SongQueryInfo = {
  instrument?: Instrument;
};

// ============ Steps ============

export type Steps = {
  id: string;
  date: string; // "YYYY-MM-DD"
  step_count: number;
  created_at: string;
  updated_at: string;
};

export type StepsFormData = {
  step_count: number;
  date: string; // "YYYY-MM-DD"
};

export type StepsQueryInfo = {
  from?: string; // "YYYY-MM-DD"
  to?: string; // "YYYY-MM-DD"
};
export type BookVolume = {
  kind: string;
  id: string;
  etag: string;
  selfLink: string;
  volumeInfo: {
    title: string;
    authors: string[];
    publisher: string;
    publishedDate: string;
    description: string;
    industryIdentifiers: {
      type: string;
      identifier: string;
    }[];
    readingModes: {
      text: boolean;
      image: boolean;
    };
    pageCount: number;
    printType: string;
    categories: string[];
    maturityRating: string;
    allowAnonLogging: boolean;
    contentVersion: string;
    panelizationSummary: {
      containsEpubBubbles: boolean;
      containsImageBubbles: boolean;
    };
    imageLinks: {
      smallThumbnail: string;
      thumbnail: string;
    };
    language: string;
    previewLink: string;
    infoLink: string;
    canonicalVolumeLink: string;
  };
  accessInfo: {
    country: string;
    viewability: string;
    embeddable: boolean;
    publicDomain: boolean;
    textToSpeechPermission: string;
    epub: {
      isAvailable: boolean;
      acsTokenLink?: string;
    };
    pdf: {
      isAvailable: boolean;
    };
    webReaderLink: string;
    accessViewStatus: string;
    quoteSharingAllowed: boolean;
  };
  searchInfo: {
    textSnippet: string;
  };
};

export type BookVolumeResponse = {
  kind: string;
  totalItems: number;
  items: BookVolume[];
};
