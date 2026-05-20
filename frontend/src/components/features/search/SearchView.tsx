"use client";

import React from "react";
import { DICT, Language } from "@/lib/i18n";
import {
  BOTTOM_CATEGORIES,
  SIDEBAR_FOLLOWING,
  SIDEBAR_POPULAR,
  Streamer,
} from "@/lib/data";
import { User, Hash, PlayCircle } from "lucide-react";

interface SearchViewProps {
  query: string;
  language: Language;
  onStreamClick: (streamer: Streamer) => void;
}

export function SearchView({
  query,
  language,
  onStreamClick,
}: SearchViewProps) {
  const t = DICT[language];
  const lowerQuery = query.toLowerCase();

  const filteredCategories = BOTTOM_CATEGORIES.filter(
    (c) =>
      c.title.toLowerCase().includes(lowerQuery) ||
      c.tags.some((tag) => tag.toLowerCase().includes(lowerQuery)),
  );

  const allStreamers = [...SIDEBAR_FOLLOWING, ...SIDEBAR_POPULAR];
  const filteredStreamers = allStreamers.filter(
    (s, index, self) =>
      self.findIndex((t) => t.id === s.id) === index &&
      (s.name.toLowerCase().includes(lowerQuery) ||
        s.game.toLowerCase().includes(lowerQuery)),
  );

  return (
    <div className="max-w-[1600px] mx-auto p-8 animate-fadeIn">
      <div className="flex items-center gap-4 mb-10 pb-6 border-b border-white/5">
        <div className="w-12 h-12 rounded-2xl bg-[#D673A9]/10 flex items-center justify-center">
          <PlayCircle size={28} className="text-[#D673A9]" />
        </div>
        <div>
          <h1 className="text-3xl font-black text-white">
            {language === "ru" ? "Результаты поиска" : "Search Results"}
          </h1>
          <p className="text-[#55555a] font-bold uppercase tracking-widest text-xs mt-1">
            {language === "ru" ? "Для запроса" : "For query"}:{" "}
            <span className="text-[#D673A9]">"{query}"</span>
          </p>
        </div>
      </div>

      {filteredStreamers.length > 0 && (
        <section className="mb-16">
          <div className="flex items-center gap-3 mb-8">
            <User size={20} className="text-[#D673A9]" />
            <h2 className="text-xl font-black text-white uppercase tracking-wider">
              {language === "ru" ? "Каналы" : "Channels"}
            </h2>
          </div>
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6">
            {filteredStreamers.map((streamer) => (
              <div
                key={streamer.id}
                className="flex flex-col items-center text-center p-6 rounded-3xl bg-[#27272a]/20 border border-white/5 hover:border-[#D673A9]/30 hover:bg-[#27272a]/40 transition-all cursor-pointer group"
                onClick={() => onStreamClick(streamer)}
              >
                <div className="relative mb-4">
                  <img
                    src={streamer.avatar}
                    alt={streamer.name}
                    className="w-24 h-24 rounded-full border-4 border-transparent group-hover:border-[#D673A9] transition-all shadow-2xl"
                  />
                  <div className="absolute bottom-0 right-0 w-6 h-6 rounded-full bg-green-500 border-4 border-[#18181b]"></div>
                </div>
                <div className="flex flex-col min-w-0 w-full">
                  <span className="font-black text-lg text-white group-hover:text-[#D673A9] transition-colors truncate">
                    {streamer.name}
                  </span>
                  <span className="text-sm text-[#55555a] font-bold uppercase truncate mt-0.5">
                    {streamer.game || "Offline"}
                  </span>
                  <div className="mt-4 flex items-center justify-center gap-1.5 px-3 py-1.5 rounded-xl bg-[#D673A9]/10 text-[#D673A9] text-xs font-black uppercase">
                    {streamer.viewers}{" "}
                    {language === "ru" ? "зрителей" : "viewers"}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </section>
      )}

      {filteredCategories.length > 0 && (
        <section>
          <div className="flex items-center gap-3 mb-8">
            <Hash size={20} className="text-blue-500" />
            <h2 className="text-xl font-black text-white uppercase tracking-wider">
              {language === "ru" ? "Категории" : "Categories"}
            </h2>
          </div>
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-8">
            {filteredCategories.map((category) => (
              <div
                key={category.id}
                className="flex flex-col group cursor-pointer"
              >
                <div className="relative aspect-[3/4] rounded-2xl overflow-hidden mb-4 bg-[#27272a] shadow-xl ring-1 ring-white/5 group-hover:ring-[#D673A9]/50 transition-all">
                  <img
                    src={category.image}
                    alt={category.title}
                    className="w-full h-full object-cover transition-transform duration-700 group-hover:scale-110"
                  />
                  <div className="absolute inset-0 bg-gradient-to-t from-black/60 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity"></div>
                </div>
                <h3 className="font-black text-white group-hover:text-[#D673A9] transition-colors truncate leading-snug">
                  {category.title}
                </h3>
                <p className="text-xs text-[#55555a] font-bold uppercase mt-1">
                  {category.viewers}
                </p>
                <div className="flex flex-wrap gap-1.5 mt-2">
                  {category.tags.slice(0, 2).map((tag) => (
                    <span
                      key={tag}
                      className="text-[9px] font-black uppercase px-2 py-0.5 rounded-md bg-white/5 text-[#55555a]"
                    >
                      {tag}
                    </span>
                  ))}
                </div>
              </div>
            ))}
          </div>
        </section>
      )}

      {filteredStreamers.length === 0 && filteredCategories.length === 0 && (
        <div className="flex flex-col items-center justify-center py-32 text-[#55555a]">
          <div className="w-24 h-24 rounded-full bg-white/5 flex items-center justify-center mb-8 border border-white/5 opacity-20">
            <Hash size={48} />
          </div>
          <span className="text-2xl font-black text-white opacity-40">
            No matches found for "{query}"
          </span>
          <p className="text-sm font-bold uppercase tracking-widest mt-2">
            Try different keywords or filters
          </p>
        </div>
      )}
    </div>
  );
}
