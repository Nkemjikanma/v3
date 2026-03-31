import { useState } from 'react'
import { BookFormModal } from "../../../components/admin/BookFormModal"
import { createFileRoute, redirect } from '@tanstack/react-router'
import { useGetBooks, useDeleteBook } from "../../../lib/hooks/useBook"
import { Book } from "../../../lib/types"

export const Route = createFileRoute('/admin/books/')({
  beforeLoad: ({ context }) => {
    if (!context.isAuthenticated) {
      throw redirect({ to: '/admin/login' })
    }
  },
  component: BooksAdmin,
})

function BooksAdmin() {
  const [modalBook, setModalBook] = useState<Book | undefined>();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const { data: books, isLoading, error } = useGetBooks()
  const deleteBook = useDeleteBook()

  if (isLoading) return <p>Loading books...</p>
  if (error) return <p className="text-red-500">{error.message}</p>

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-outfitSemiBold">Books</h1>
        <button
          onClick={() => { setModalBook(undefined); setIsModalOpen(true); }}
          className="px-4 py-2 bg-black text-white rounded text-sm"
        >
          Add Book
        </button>
      </div>

      {!books?.length ? (
        <p className="text-gray-500">No books yet.</p>
      ) : (
        <table className="w-full text-left text-sm">
          <thead className="border-b">
            <tr>
              <th className="py-2">Title</th>
              <th className="py-2">Author</th>
              <th className="py-2">Status</th>
              <th className="py-2">Category</th>
              <th className="py-2">Actions</th>
            </tr>
          </thead>
          <tbody>
            {books.map((book) => (
              <tr key={book.id} className="border-b">
                <td className="py-2">{book.title}</td>
                <td className="py-2">{book.author}</td>
                <td className="py-2">{book.status}</td>
                <td className="py-2">{book.category}</td>
                <td className="py-2 flex gap-2">
                  <button
                    onClick={() => { setModalBook(book); setIsModalOpen(true); }}
                    className="text-blue-600 hover:underline"
                  >
                    Edit
                  </button>
                  <button
                    onClick={() => deleteBook.mutate(book.id)}
                    disabled={deleteBook.isPending}
                    className="text-red-600 hover:underline"
                  >
                    Delete
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}

      <BookFormModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        book={modalBook}
      />
    </div>
  )
}
