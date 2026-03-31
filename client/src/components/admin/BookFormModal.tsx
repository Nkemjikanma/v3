import { useState, useEffect } from "react"
import { Book, BookCategory, BookStatus } from "../../lib/types"
import { useAddBook, useUpdateBook } from "../../lib/hooks/useBook"
type BookFormModalProps = {
	isOpen: boolean;
	onClose: () => void;
	book?: Book; // if present, edit mode; if absent, add mode
}


export function BookFormModal({ isOpen, onClose, book }: BookFormModalProps) {
	const [title, setTitle] = useState("");
	const [author, setAuthor] = useState("");
	const [status, setStatus] = useState<BookStatus>("reading");
	const [category, setCategory] = useState<BookCategory>("technical");
	const [error, setError] = useState("");

	const addBook = useAddBook();
	const updateBook = useUpdateBook();

	const isPending = addBook.isPending || updateBook.isPending;

	useEffect(() => {
		if (book) {
			setTitle(book.title);
			setAuthor(book.author);
			setStatus(book.status);
			setCategory(book.category);
		} else {
			setTitle("");
			setAuthor("");
			setStatus("reading");
			setCategory("technical");
		}
		setError("");
	}, [book, isOpen]);

	const handleSubmit = (e: React.FormEvent) => {
		e.preventDefault();
		setError("");

		if (!title.trim() || !author.trim()) {
			setError("Title and author are required");
			return;
		}

		const onSuccess = () => onClose();
		const onError = (err: Error) => setError(err.message);

		if (book) {
			updateBook.mutate(
				{ id: book.id, book: { title, author, status, category } },
				{ onSuccess, onError }
			);
		} else {
			addBook.mutate(
				{ title, author, status, category },
				{ onSuccess, onError }
			);
		}
	};

	if (!isOpen) return null;

	return (
		<div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
			<form
				onSubmit={handleSubmit}
				className="bg-white p-6 rounded space-y-4 w-full max-w-md"
			>
				<h2 className="text-xl font-outfitSemiBold">
					{book ? "Edit Book" : "Add Book"}
				</h2>

				<div className="space-y-1">
					<label className="text-sm">Title</label>
					<input
						type="text"
						value={title}
						onChange={(e) => setTitle(e.target.value)}
						className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none
  focus:border-black"
					/>
				</div>

				<div className="space-y-1">
					<label className="text-sm">Author</label>
					<input
						type="text"
						value={author}
						onChange={(e) => setAuthor(e.target.value)}
						className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none
  focus:border-black"
					/>
				</div>

				<div className="space-y-1">
					<label className="text-sm">Status</label>
					<select
						value={status}
						onChange={(e) => setStatus(e.target.value as BookStatus)}
						className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none
  focus:border-black"
					>
						<option value="reading">Reading</option>
						<option value="finished">Finished</option>
					</select>
				</div>

				<div className="space-y-1">
					<label className="text-sm">Category</label>
					<select
						value={category}
						onChange={(e) => setCategory(e.target.value as BookCategory)}
						className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none
  focus:border-black"
					>
						<option value="technical">Technical</option>
						<option value="leisure">Leisure</option>
						<option value="music">Music</option>
					</select>
				</div>

				{error && <p className="text-red-500 text-sm">{error}</p>}

				<div className="flex gap-3 justify-end">
					<button
						type="button"
						onClick={onClose}
						className="px-4 py-2 border border-gray-300 rounded text-sm"
					>
						Cancel
					</button>
					<button
						type="submit"
						disabled={isPending}
						className="px-4 py-2 bg-black text-white rounded text-sm disabled:opacity-50"
					>
						{isPending ? "Saving..." : book ? "Update" : "Add"}
					</button>
				</div>
			</form>
		</div>
	);
}
