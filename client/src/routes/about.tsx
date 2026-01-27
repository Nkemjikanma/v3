import { createRoute } from "@tanstack/react-router";
import { useGetReads } from "../lib/hooks/useGetReads";
import { Route as rootRoute } from "./__root";

export const Route = createRoute({
  getParentRoute: () => rootRoute,
  path: "/about",
  component: About,
});

const guitarSongs = [{ name: "Learning in progress...", artist: "" }];

function About() {
  const { data: booksData, isLoading, error } = useGetReads();

  const books = booksData
    ?.flat()
    .filter((item) => item?.items?.[0]?.volumeInfo)
    .map((item) => ({
      title: item.items[0].volumeInfo.title,
      author: item.items[0].volumeInfo.authors?.join(", "),
    }));
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
        <ul className="mt-4 flex flex-col gap-3">
          {isLoading ? (
            <li className="text-black text-sm">Loading...</li>
          ) : error ? (
            <li className="text-black text-sm">Could not load books</li>
          ) : books && books.length > 0 ? (
            books.map((book) => (
              <li key={book.title} className="flex flex-col">
                <span className="font-outfitRegular">{book.title}</span>
                <span className="text-black text-sm">{book.author}</span>
              </li>
            ))
          ) : (
            <li className="text-black text-sm">No books at the moment</li>
          )}
        </ul>
      </section>

      {/* Guitar */}
      <section>
        <h2 className="text-sm font-outfitSemiBold text-black uppercase tracking-wide">
          Guitar
        </h2>
        <p className="mt-2 text-black text-sm">Songs I'm learning to play</p>
        <ul className="mt-4 flex flex-col gap-2">
          {guitarSongs.map((song, index) => (
            <li key={index} className="flex justify-between items-baseline">
              <span className="font-outfitRegular">{song.name}</span>
              {song.artist && (
                <span className="text-black text-sm">{song.artist}</span>
              )}
            </li>
          ))}
        </ul>
      </section>
    </div>
  );
}
