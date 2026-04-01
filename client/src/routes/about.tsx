import { createFileRoute } from "@tanstack/react-router";
import { useGetBooks } from "../lib/hooks/useBook";
import { useGetSongs } from "../lib/hooks/useSongs";
import StepsChart from "../components/StepsChart";

export const Route = createFileRoute("/about")({
  component: About,
});

function SkeletonLoader() {
  return (
    <>
      <li className="h-4 w-48 bg-gray-200 animate-pulse rounded" />
      <li className="h-4 w-36 bg-gray-200 animate-pulse rounded" />
      <li className="h-4 w-40 bg-gray-200 animate-pulse rounded" />
    </>
  );
}

function About() {
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
      <section className="bg-gray-50 rounded-lg p-4">
        <h2 className="text-sm font-outfitSemiBold text-black uppercase tracking-wide">
          Currently Reading{books && books.length > 0 ? ` (${books.length})` : ""}
        </h2>
        <ul className="mt-4 flex flex-col gap-3 max-h-64 overflow-y-auto">
          {isLoading ? (
            <SkeletonLoader />
          ) : error ? (
            <li className="text-gray-400 text-sm">Could not load books</li>
          ) : books && books.length > 0 ? (
            books.map((book) => (
              <li key={book.id} className="flex flex-col border-l-2 border-gray-300 pl-3">
                <span className="font-outfitRegular">{book.title}</span>
                <div className="flex items-center gap-2">
                  <span className="text-gray-500 text-sm">{book.author}</span>
                  <span className="text-xs text-gray-400 bg-gray-200 px-1.5 py-0.5 rounded">
                    {book.category}
                  </span>
                </div>
              </li>
            ))
          ) : (
            <li className="text-gray-400 text-sm">No books at the moment</li>
          )}
        </ul>
      </section>

      {/* Songs */}
      <section className="bg-gray-50 rounded-lg p-4">
        <h2 className="text-sm font-outfitSemiBold text-black uppercase tracking-wide">
          Music is life{songs && songs.length > 0 ? ` (${songs.length})` : ""}
        </h2>
        <p className="mt-1 text-gray-500 text-sm">Songs I'm learning to play</p>
        <ul className="mt-4 flex flex-col gap-2 max-h-64 overflow-y-auto">
          {songsLoading ? (
            <SkeletonLoader />
          ) : songs && songs.length > 0 ? (
            songs.map((song) => (
              <li key={song.id} className="flex justify-between items-center border-l-2 border-gray-300 pl-3 py-1">
                <div className="flex flex-col">
                  <span className="font-outfitRegular">{song.title}</span>
                  <span className="text-gray-500 text-sm">{song.artist}</span>
                </div>
                <span className="text-xs text-gray-400 bg-gray-200 px-1.5 py-0.5 rounded">
                  {song.instrument}
                </span>
              </li>
            ))
          ) : (
            <li className="text-gray-400 text-sm">No songs yet</li>
          )}
        </ul>
      </section>

      <section className="bg-gray-50 rounded-lg p-4">
        <h2 className="text-sm font-outfitSemiBold text-black uppercase tracking-wide">
          I run sometimes
        </h2>
        <div className="mt-4">
          <StepsChart />
        </div>
      </section>
    </div>
  );
}
