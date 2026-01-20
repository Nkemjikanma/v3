import { useQuery } from "@tanstack/react-query";

export type BlogPost = {
  title: string;
  slug: string;
  date: string;
  url: string;
  excerpt?: string;
};

// Mock data for now - replace with actual API call when ready
const mockPosts: BlogPost[] = [
  // {
  // 	title: "Getting started with Rust and WebAssembly",
  // 	slug: "rust-wasm-intro",
  // 	date: "Jan 2026",
  // 	url: "https://nkem.dev/blog/rust-wasm-intro",
  // },
  // {
  // 	title: "Building type-safe APIs with Hono",
  // 	slug: "hono-type-safe-apis",
  // 	date: "Dec 2025",
  // 	url: "https://nkem.dev/blog/hono-type-safe-apis",
  // },
  // {
  // 	title: "My journey into Solidity development",
  // 	slug: "solidity-journey",
  // 	date: "Nov 2025",
  // 	url: "https://nkem.dev/blog/solidity-journey",
  // },
];

const fetchPosts = async (): Promise<BlogPost[]> => {
  // TODO: Replace with actual API call
  // const response = await fetch('https://nkem.dev/api/posts');
  // return response.json();

  // Simulate network delay
  await new Promise((resolve) => setTimeout(resolve, 500));
  return mockPosts;
};

export const useGetPosts = () => {
  return useQuery({
    queryKey: ["get-blog-posts"],
    queryFn: fetchPosts,
  });
};
