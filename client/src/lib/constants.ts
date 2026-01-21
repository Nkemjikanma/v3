export const introItems = [
  {
    value: "Father and husband",
  },
  {
    value: "Fullstack developer",
  },
  {
    value: "Guitarist and songwriter",
  },
];

export const experiences = [
  {
    discStyle: "Present",
    name: "Freelance",
    duration: "Oct. 23 - ",
  },
  {
    discStyle: "Past",
    name: "Navro",
    duration: "Nov. 2022 - Sept. 2024",
  },
  {
    discStyle: "Past",
    name: "Thames composite",
    duration: "Jan. 2021 - Oct. 2021",
  },
];

export const personal_projects = [
  {
    name: "Pepino",
    link: "https://crates.io/crates/pepino",
    icon: "🥒",
    description:
      "A fullstack scaffolder for modern Rust + TypeScript web applications",
  },
  {
    name: "Coco",
    link: "https://app.towns.com/bot/0x1aFb580ED1AdE828431419B1bd54Aeb1133B7e46",
    icon: "🤖",
    description: "An ENS domains management bot built for Towns app",
  },
];

export const bookShelf = {
  current: [
    {
      title: "The road to the country",
      author: "Chigozie Obioma",
    },
    {
      title: "Javascript: The definitive guide",
      author: "David Flanagan",
    },
    {
      title: "Mickey Baker's Complete Course in Jazz Guitar: Book 1",
      author: "Mickey Baker",
    },
    {
      title: "Sankofa",
      author: "Chibundu Onuzo",
    },
  ],
  previous: [{}],
};

export const toolings = [
  {
    tool: "Rust",
    link: "/images/logos/rust.svg",
  },
  {
    tool: "Typescript",
    link: "/images/logos/typescript.svg",
  },
  {
    tool: "Solidity",
    link: "/images/logos/solidity.svg",
  },
  {
    tool: "foundry",
    link: "/images/logos/foundry.png",
  },
  {
    tool: "NodeJS",
    link: "/images/logos/node.svg",
  },
  {
    tool: "React",
    link: "/images/logos/react.svg",
  },
  {
    tool: "NextJS",
    link: "/images/logos/next.svg",
  },
  {
    tool: "TailwindCSS",
    link: "/images/logos/tailwind.svg",
  },
  {
    tool: "Prisma",
    link: "/images/logos/prisma.svg",
  },
  {
    tool: "Hono",
    link: "/images/logos/hono.png",
  },
  {
    tool: "Reac Testing Library",
    link: "/images/logos/rtl.png",
  },
  {
    tool: "Jest",
    link: "/images/logos/jest.svg",
  },
  {
    tool: "MSW",
    link: "/images/logos/msw.svg",
  },
];

export const socials = [
  {
    name: "Mail",
    logo: "/images/logos/email.svg",
    profile: "mailto:contact@nkem.dev",
  },
  {
    name: "Github",
    logo: "/images/logos/github.svg",
    profile: "https://github.com/Nkemjikanma",
  },
  {
    name: "Linkedin",
    logo: "/images/logos/linkedin.svg",
    profile: "https://www.linkedin.com/in/nkemjika-ohanyere/",
  },
  // {
  //   name: "Farcaster",
  //   logo: "/images/logos/farcaster.svg",
  //   profile: "https://warpcast.com/nkemjika",
  // },
  {
    name: "ENS",
    logo: "/images/logos/ens1.svg",
    profile: "https://app.ens.domains/nkemjika.eth",
  },
  {
    name: "Towns",
    logo: "/images/logos/towns.svg",
    profile:
      "https://app.towns.com/t/0x16c26e46624ebfd0929c0b0a2d0f51ff1514eb31/",
  },
];

export const ens = {
  logo: "/images/logos/ens.svg",
  domains: ["nkemjika.eth", "nkem.eth", "keyof.eth"],
};

export const BOOKS_API = import.meta.env.VITE_BOOKS_API_KEY;
