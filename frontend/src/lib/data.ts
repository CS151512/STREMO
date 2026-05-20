export interface Streamer {
  id: number;
  name: string;
  game: string;
  viewers: string;
  avatar: string;
  isLive: boolean;
  title?: string;
  tags?: string[];
}

export interface Category {
  id: number;
  title: string;
  viewers: string;
  tags: string[];
  image: string;
  desc?: string;
}

export const SIDEBAR_FOLLOWING: Streamer[] = [
  {
    id: 1,
    name: "Clemovitch",
    game: "Just Chatting",
    viewers: "17.3K",
    avatar: "https://i.pravatar.cc/150?u=1",
    isLive: true,
  },
  {
    id: 2,
    name: "Glorious_E",
    game: "The Witcher 3: Wild Hunt",
    viewers: "56.5K",
    avatar: "https://i.pravatar.cc/150?u=2",
    isLive: true,
  },
  {
    id: 3,
    name: "Camy",
    game: "Call of Duty: Warzone",
    viewers: "1.3K",
    avatar: "https://i.pravatar.cc/150?u=3",
    isLive: true,
  },
  {
    id: 4,
    name: "BLASTPremier",
    game: "",
    viewers: "Offline",
    avatar: "https://i.pravatar.cc/150?u=4",
    isLive: false,
  },
  {
    id: 5,
    name: "fubgun",
    game: "",
    viewers: "Offline",
    avatar: "https://i.pravatar.cc/150?u=5",
    isLive: false,
  },
];

export const SIDEBAR_POPULAR: Streamer[] = [
  {
    id: 6,
    name: "KaiCenat",
    game: "Just Chatting",
    viewers: "56.5K",
    avatar: "https://i.pravatar.cc/150?u=6",
    isLive: true,
  },
  {
    id: 7,
    name: "Jynxzi",
    game: "I'm Only Sleeping",
    viewers: "9.3K",
    avatar: "https://i.pravatar.cc/150?u=7",
    isLive: true,
  },
  {
    id: 8,
    name: "IShowSpeed",
    game: "IRL",
    viewers: "12.4K",
    avatar: "https://i.pravatar.cc/150?u=8",
    isLive: true,
  },
  {
    id: 9,
    name: "Caedrel",
    game: "Hollow Knight",
    viewers: "18.6K",
    avatar: "https://i.pravatar.cc/150?u=9",
    isLive: true,
  },
  {
    id: 10,
    name: "faxuty",
    game: "Fortnite",
    viewers: "9.9K",
    avatar: "https://i.pravatar.cc/150?u=10",
    isLive: true,
  },
];

export const SIDEBAR_CATEGORIES: Category[] = [
  {
    id: 1,
    title: "Politics",
    desc: "IRL",
    viewers: "56.5K",
    image: "https://picsum.photos/seed/pol/100/100",
    tags: ["IRL"],
  },
  {
    id: 2,
    title: "Just Chatting",
    desc: "IRL",
    viewers: "56.5K",
    image: "https://picsum.photos/seed/chat/100/100",
    tags: ["IRL"],
  },
  {
    id: 3,
    title: "Counter-Strike",
    desc: "FPS, Shooter",
    viewers: "56.5K",
    image: "https://picsum.photos/seed/csgo/100/100",
    tags: ["FPS", "Shooter"],
  },
  {
    id: 4,
    title: "Path of Exile 2",
    desc: "RPG, Action",
    viewers: "56.5K",
    image: "https://picsum.photos/seed/poe/100/100",
    tags: ["RPG", "Action"],
  },
  {
    id: 5,
    title: "The Witcher 3: W...",
    desc: "RPG",
    viewers: "56.5K",
    image: "https://picsum.photos/seed/witcher_icon/100/100",
    tags: ["RPG"],
  },
];

export const RECOMMENDATIONS = [
  {
    id: 1,
    title: "Direction Kear Morhen | Marathon THE WITCHER 3 + DL...",
    channel: "Glorious_E",
    game: "The Witcher 3: Wild Hunt",
    viewers: "56.5K viewers",
    image: "https://picsum.photos/seed/witcher_stream/600/338",
    avatar: "https://i.pravatar.cc/150?u=2",
  },
  {
    id: 2,
    title: "⭐#1 RANKED POV⭐30% OFF AL...",
    channel: "Camy",
    game: "Call of Duty: Warzone",
    viewers: "1.3K viewers",
    image: "https://picsum.photos/seed/cod/600/338",
    avatar: "https://i.pravatar.cc/150?u=3",
  },
  {
    id: 3,
    title: "ROAD TO GLOBAL ELITE | CS2 CHILL STREAM",
    channel: "s1mple_fan",
    game: "Counter-Strike 2",
    viewers: "24.1K viewers",
    image: "https://picsum.photos/seed/cs2_stream/600/338",
    avatar: "https://i.pravatar.cc/150?u=4",
  },
  {
    id: 4,
    title: "Just chatting and reacting to new trailers",
    channel: "Pokimane_fan",
    game: "Just Chatting",
    viewers: "32.8K viewers",
    image: "https://picsum.photos/seed/chat_stream/600/338",
    avatar: "https://i.pravatar.cc/150?u=5",
  },
];

export const BOTTOM_CATEGORIES: Category[] = [
  {
    id: 1,
    title: "Just Chatting",
    viewers: "232K viewers",
    tags: ["IRL", "Casual"],
    image: "https://picsum.photos/seed/justchat/400/533",
  },
  {
    id: 2,
    title: "IRL",
    viewers: "142K viewers",
    tags: ["IRL", "Adventure"],
    image: "https://picsum.photos/seed/irl_cat/400/533",
  },
  {
    id: 3,
    title: "Minecraft",
    viewers: "67.9K viewers",
    tags: ["Simulation"],
    image: "https://picsum.photos/seed/mine/400/533",
  },
  {
    id: 4,
    title: "Dota 2",
    viewers: "59.7K viewers",
    tags: ["Strategy", "MOBA"],
    image: "https://picsum.photos/seed/dota2/400/533",
  },
  {
    id: 5,
    title: "Counter-Strike",
    viewers: "21.2K viewers",
    tags: ["FPS", "Shooter"],
    image: "https://picsum.photos/seed/cs2/400/533",
  },
  {
    id: 6,
    title: "Path of Exile 2",
    viewers: "128K viewers",
    tags: ["Action", "RPG"],
    image: "https://picsum.photos/seed/poe2/400/533",
  },
  {
    id: 7,
    title: "Politics",
    viewers: "45.1K viewers",
    tags: ["IRL"],
    image: "https://picsum.photos/seed/polcat/400/533",
  },
  {
    id: 8,
    title: "The Witcher 3",
    viewers: "32.4K viewers",
    tags: ["RPG"],
    image: "https://picsum.photos/seed/w3cat/400/533",
  },
];
