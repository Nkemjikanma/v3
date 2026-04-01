import { createFileRoute } from "@tanstack/react-router";
import { useGetBooks } from "../lib/hooks/useBook"
import { useGetSongs } from "../lib/hooks/useSongs"

export const Route = createFileRoute("/about")({
  component: About,
});


function About() {
  // const { data: booksData, isLoading, error } = useGetReads();
  const { data: books, isLoading, error } = useGetBooks({ status: "reading" });
  const { data: songs, isLoading: songsLoading } = useGetSongs();

  return (
    <div className="flex flex-col gap-12">
      {/* Personal */}
      <section>
        <p className="text-black leading-relaxed">
          Father and husband. Guitarist and songwriter.
          <br />I am currently working as a Freelance Software engineer. I also
          enjoy reading - mostly fiction, creating music, and building things
          that hopefully solve real problems.
        </p>
      </section>

      {/* Currently Reading */}
      <section>
        <h2 className="text-sm font-outfitSemiBold text-black uppercase tracking-wide">
          Currently Reading
        </h2>
        <ul className="mt-4 flex flex-col gap-3 max-h-64 overflow-y-auto">
          {isLoading ? (
            <li className="text-black text-sm">Loading...</li>
          ) : error ? (
            <li className="text-black text-sm">Could not load books</li>
          ) : books && books.length > 0 ? (
            books.map((book) => (
              <li key={book.id} className="flex flex-col">
                <span className="font-outfitRegular">{book.title}</span>
                <span className="text-black text-sm">{book.author}</span>
              </li>
            ))
          ) : (
            <li className="text-black text-sm">No books at the moment</li>
          )}
        </ul>
      </section>

      {/* Songs */}
      <section>
        <h2 className="text-sm font-outfitSemiBold text-black uppercase tracking-wide">
          Music is life
        </h2>
        <p className="mt-2 text-black text-sm">Songs I'm learning to play</p>
        <ul className="mt-4 flex flex-col gap-2 max-h-64 overflow-y-auto">
          {songsLoading ? (
            <li className="text-black text-sm">Loading...</li>
          ) : songs && songs.length > 0 ? (
            songs.map((song) => (
              <li key={song.id} className="flex justify-between items-baseline">
                <span className="font-outfitRegular">{song.title}</span>
                <span className="text-black text-sm">
                  {song.artist} · {song.instrument}
                </span>
              </li>
            ))
          ) : (
            <li className="text-black text-sm">No songs yet</li>
          )}
        </ul>
      </section>
    </div>
  );
}
