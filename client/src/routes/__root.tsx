import {
  createRootRoute,
  Link,
  Outlet,
  useLocation,
} from "@tanstack/react-router";

export const Route = createRootRoute({
  component: RootLayout,
});

function Header() {
  const location = useLocation();
  const isAbout = location.pathname === "/about";

  return (
    <header className="flex justify-between items-center">
      <Link
        to="/"
        className="font-outfitSemiBold text-lg hover:text-gray-600 transition-colors"
      >
        nkem
      </Link>
      <nav className="flex gap-4 sm:gap-6 text-sm">
        <Link
          to="/about"
          className={`transition-colors ${isAbout ? "text-gray-900" : "text-black hover:text-gray-900"}`}
        >
          about
        </Link>
        <a
          href="https://nkem.dev/blog"
          target="_blank"
          rel="noreferrer"
          className="text-black hover:text-gray-900 transition-colors"
        >
          blog
        </a>
        <a
          href="https://esemese.xyz"
          target="_blank"
          rel="noreferrer"
          className="text-black hover:text-gray-900 transition-colors"
        >
          photos
        </a>
      </nav>
    </header>
  );
}

function Footer() {
  return (
    <footer className="mt-12 pt-6 border-t border-gray-100">
      <p className="text-black text-sm">
        🧌 &copy; {new Date().getFullYear()} Nkemjika
      </p>
    </footer>
  );
}

function RootLayout() {
  return (
    <div className="relative min-h-[100dvh] w-screen flex items-center justify-center font-outfitRegular min-w-[22em] bg-white">
      <div className="relative min-h-[100dvh] w-full max-w-2xl px-6 py-12 flex flex-col">
        <Header />
        <main className="flex-1 mt-12">
          <Outlet />
        </main>
        <Footer />
      </div>
    </div>
  );
}
