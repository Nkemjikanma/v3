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
	items: BookVolume[]
}
