import { useState } from 'react'
import { createFileRoute } from '@tanstack/react-router'
import { useGetSongs, useDeleteSong } from "../../../lib/hooks/useSongs"
import { Song } from "../../../lib/types"
import SongFormModal from "../../../components/admin/SongFormModal"

export const Route = createFileRoute('/admin/songs/')({
  component: SongsAdmin,
})

function SongsAdmin() {
  const [modalSong, setModalSong] = useState<Song | undefined>();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const { data: songs, isLoading, error } = useGetSongs()
  const deleteSong = useDeleteSong()

  if (isLoading) return <p>Loading songs...</p>
  if (error) return <p className="text-red-500">{error.message}</p>

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-outfitSemiBold">Songs</h1>
        <button
          onClick={() => { setModalSong(undefined); setIsModalOpen(true); }}
          className="px-4 py-2 bg-black text-white rounded text-sm"
        >
          Add Song
        </button>
      </div>

      {!songs?.length ? (
        <p className="text-gray-500">No songs yet.</p>
      ) : (
        <table className="w-full text-left text-sm">
          <thead className="border-b">
            <tr>
              <th className="py-2">Title</th>
              <th className="py-2">Artist</th>
              <th className="py-2">Instrument</th>
              <th className="py-2">Started</th>
              <th className="py-2">Actions</th>
            </tr>
          </thead>
          <tbody>
            {songs.map((song) => (
              <tr key={song.id} className="border-b">
                <td className="py-2">{song.title}</td>
                <td className="py-2">{song.artist}</td>
                <td className="py-2">{song.instrument}</td>
                <td className="py-2">{song.started_learning_at}</td>
                <td className="py-2 flex gap-2">
                  <button
                    onClick={() => { setModalSong(song); setIsModalOpen(true); }}
                    className="text-blue-600 hover:underline"
                  >
                    Edit
                  </button>
                  <button
                    onClick={() => deleteSong.mutate(song.id)}
                    disabled={deleteSong.isPending}
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

      <SongFormModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        song={modalSong}
      />
    </div>
  )
}
