import { createRoute } from "@tanstack/react-router";
import { experiences, personal_projects, socials } from "../lib/constants";
import { useGetPosts } from "../lib/hooks/useGetPosts";
import { Route as rootRoute } from "./__root";

export const Route = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: Landing,
});

function Landing() {
  const { data: posts, isLoading } = useGetPosts();

  return (
    <div className="flex flex-col gap-12">
      {/* Intro */}
      <section>
        <h1 className="text-2xl font-outfitSemiBold">Nkemjika Ohanyere</h1>
        <p className="text-black mt-1">Fullstack Software Engineer</p>
        <p className="text-black mt-4 leading-relaxed">
          I know, you can call me <span className="font-outfitBold">Kemji</span>
          . I am a software engineer with a passion for building technologies
          that broaden access and improve financial services. I am currently
          working as a freelance sofware engineer building with Rust, Solidity &
          TypeScript.
        </p>
        <div className="flex gap-6 mt-6">
          <p>You can find me on: </p>
          {socials.map((social) => (
            <a
              key={social.name}
              href={social.profile}
              target="_blank"
              rel="noreferrer"
              className="opacity-80 hover:opacity-100 transition-opacity"
              title={social.name}
            >
              <img src={social.logo} alt={social.name} className="w-5 h-5" />
            </a>
          ))}
          <></>
        </div>
      </section>

      {/* Projects */}
      <section>
        <h2 className="text-sm font-outfitSemiBold text-black uppercase tracking-wide">
          Projects
        </h2>
        <ul className="mt-4 flex flex-col gap-4 sm:gap-3">
          {personal_projects.map((project) => (
            <li key={project.name}>
              <a
                href={project.link}
                target="_blank"
                rel="noreferrer"
                className="flex flex-col sm:flex-row sm:justify-between sm:items-baseline group"
              >
                <span className="font-outfitRegular group-hover:text-gray-600 transition-colors">
                  {project.name}
                </span>
                <span className="text-black text-sm mt-1 sm:mt-0">
                  {project.description}
                </span>
              </a>
            </li>
          ))}
        </ul>
      </section>

      {/* Experience */}
      <section>
        <h2 className="text-sm font-outfitSemiBold text-black uppercase tracking-wide">
          Experience
        </h2>
        <ul className="mt-4 flex flex-col gap-3 sm:gap-2">
          {experiences.map((exp) => (
            <li
              key={exp.name}
              className="flex flex-col sm:flex-row sm:justify-between sm:items-baseline"
            >
              <span className="font-outfitRegular flex items-center gap-2">
                {exp.discStyle === "Present" ? (
                  <span className="relative flex h-2 w-2">
                    <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
                    <span className="relative inline-flex rounded-full h-2 w-2 bg-green-500"></span>
                  </span>
                ) : (
                  <span className="inline-flex rounded-full h-2 w-2 bg-gray-300"></span>
                )}
                {exp.name}
              </span>
              <span className="text-black text-sm mt-1 sm:mt-0 pl-4 sm:pl-0">
                {exp.duration}
              </span>
            </li>
          ))}
        </ul>
      </section>

      {/* Writing */}
      <section>
        <div className="flex justify-between items-baseline">
          <h2 className="text-sm font-outfitSemiBold text-black uppercase tracking-wide">
            Writing
          </h2>
          <a
            href="https://nkem.dev/blog"
            target="_blank"
            rel="noreferrer"
            className="text-sm text-black hover:text-gray-600 transition-colors"
          >
            view all →
          </a>
        </div>
        <ul className="mt-4 flex flex-col gap-4 sm:gap-3">
          {isLoading ? (
            <li className="text-black text-sm">Loading posts...</li>
          ) : posts && posts.length > 0 ? (
            posts.slice(0, 3).map((post) => (
              <li key={post.slug}>
                <a
                  href={post.url}
                  target="_blank"
                  rel="noreferrer"
                  className="flex flex-col sm:flex-row sm:justify-between sm:items-baseline group"
                >
                  <span className="font-outfitRegular group-hover:text-gray-600 transition-colors">
                    {post.title}
                  </span>
                  <span className="text-black text-sm mt-1 sm:mt-0">
                    {post.date}
                  </span>
                </a>
              </li>
            ))
          ) : (
            <li className="text-black text-sm">No posts yet</li>
          )}
        </ul>
      </section>
    </div>
  );
}
