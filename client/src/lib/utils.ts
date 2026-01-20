import type { BookVolumeResponse } from "./types";
import { bookShelf, BOOKS_API } from "./constants";

export const getCurrentlyReadingBooks = async (): Promise<BookVolumeResponse[]> => {
	const books = bookShelf.current;
	const responses = await Promise.all(
		books.map(async (book) => {
			const response = await fetch(
				`https://www.googleapis.com/books/v1/volumes?q=${encodeURIComponent(book.title)}+inauthor:${encodeURIComponent(book.author)}&key=${BOOKS_API}`
			);
			return response.json() as Promise<BookVolumeResponse>;
		})
	);
	return responses;
};
