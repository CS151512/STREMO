"use client";

import React, { useEffect, useRef } from "react";
import {
  Search,
  X,
  TrendingUp,
  User,
  Play,
  Hash,
  Clock,
  ArrowRight,
} from "lucide-react";
import { DICT, Language } from "@/lib/i18n";
import {
  BOTTOM_CATEGORIES,
  SIDEBAR_FOLLOWING,
  SIDEBAR_POPULAR,
  Streamer,
} from "@/lib/data";

interface SearchOverlayProps {
  isOpen: boolean;
  onClose: () => void;
  query: string;
  setQuery: (query: string) => void;
  language: Language;
  onStreamClick: (streamer: Streamer) => void;
}

export function SearchOverlay({
  isOpen,
  onClose,
  query,
  setQuery,
  language,
  onStreamClick,
}: SearchOverlayProps) {
  const t = DICT[language];
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (isOpen) {
      document.body.style.overflow = "hidden";
      setTimeout(() => inputRef.current?.focus(), 100);
    } else {
      document.body.style.overflow = "auto";
    }
  }, [isOpen]);

  useEffect(() => {
    const handleEsc = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    window.addEventListener("keydown", handleEsc);
    return () => window.removeEventListener("keydown", handleEsc);
  }, [onClose]);

  if (!isOpen) return null;

  const lowerQuery = query.toLowerCase();

  const allStreamers = [...SIDEBAR_FOLLOWING, ...SIDEBAR_POPULAR];
  const filteredStreamers = allStreamers
    .filter(
      (s, index, self) =>
        self.findIndex((t) => t.id === s.id) === index && // Unique by id
        (s.name.toLowerCase().includes(lowerQuery) ||
          s.game.toLowerCase().includes(lowerQuery)),
    )
    .slice(0, 6);

  const filteredCategories = BOTTOM_CATEGORIES.filter(
    (c) =>
      c.title.toLowerCase().includes(lowerQuery) ||
      c.tags.some((tag) => tag.toLowerCase().includes(lowerQuery)),
  ).slice(0, 8);

  const trendingCategories = BOTTOM_CATEGORIES.slice(0, 4);
  const recentSearches = [
    language === "ru" ? "Киберспорт" : "Esports",
    "Minecraft",
    "ASMR",
    language === "ru" ? "Разговоры" : "Just Chatting",
  ];

  return (
    <div className="fixed inset-0 z-[100] bg-[#0e0e10]/95 backdrop-blur-md flex flex-col animate-fadeIn">
      {/* Search Header */}
      <div className="h-24 flex items-center px-6 border-b border-white/5 bg-[#131315]/80">
        <div className="flex-1 max-w-5xl mx-auto flex items-center gap-6">
          <div
            className="flex items-center gap-2 cursor-pointer shrink-0"
            onClick={onClose}
          >
            <svg
              viewBox="0 0 100 100"
              className="w-10 h-10 text-[#D673A9] fill-current"
            >
              <path d="M50 0 L58 12 L72 8 L76 22 L90 25 L88 39 L100 50 L88 61 L90 75 L76 78 L72 92 L58 88 L50 100 L42 88 L28 92 L24 78 L10 75 L12 61 L0 50 L12 39 L10 25 L24 22 L28 8 L42 12 Z" />
            </svg>
            <span className="text-2xl font-black tracking-wider hidden sm:block">
              STREMO
            </span>
          </div>

          <div className="flex-1 flex items-center gap-4 h-14 px-6 rounded-full bg-[#18181b] border border-white/10 focus-within:border-[#D673A9] focus-within:ring-4 focus-within:ring-[#D673A9]/10 transition-all shadow-2xl">
            <Search size={22} className="text-[#a1a1aa]" />
            <input
              ref={inputRef}
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder={t.search}
              className="w-full bg-transparent outline-none text-xl font-bold text-white placeholder-[#55555a]"
            />
            {query && (
              <button
                onClick={() => setQuery("")}
                className="p-1.5 hover:bg-white/10 rounded-full transition-colors"
              >
                <X size={20} className="text-[#a1a1aa]" />
              </button>
            )}
          </div>

          <button
            onClick={onClose}
            className="h-12 w-12 flex items-center justify-center rounded-full bg-white/5 hover:bg-white/10 text-white transition-all active:scale-90 border border-white/5"
          >
            <X size={24} />
          </button>
        </div>
      </div>

      {/* Results / Suggestions */}
      <div className="flex-1 overflow-y-auto custom-scrollbar px-6 py-10">
        <div className="max-w-5xl mx-auto w-full">
          {!query ? (
            <div className="grid grid-cols-1 lg:grid-cols-12 gap-12 animate-slideUp">
              {/* Left Column: Recent & Trending */}
              <div className="lg:col-span-4 space-y-10">
                <section>
                  <h2 className="text-xs font-black uppercase tracking-[0.2em] text-[#55555a] mb-6 flex items-center gap-2">
                    <Clock size={14} />{" "}
                    {language === "ru" ? "Недавние запросы" : "Recent Searches"}
                  </h2>
                  <div className="flex flex-col gap-1">
                    {recentSearches.map((search) => (
                      <button
                        key={search}
                        onClick={() => setQuery(search)}
                        className="flex items-center justify-between p-3 rounded-xl hover:bg-white/5 text-left group transition-colors"
                      >
                        <span className="font-bold text-[#a1a1aa] group-hover:text-white">
                          {search}
                        </span>
                        <ArrowRight
                          size={14}
                          className="text-[#a1a1aa] opacity-0 group-hover:opacity-100 -translate-x-2 group-hover:translate-x-0 transition-all"
                        />
                      </button>
                    ))}
                  </div>
                </section>

                <section>
                  <h2 className="text-xs font-black uppercase tracking-[0.2em] text-[#55555a] mb-6 flex items-center gap-2">
                    <TrendingUp size={14} />{" "}
                    {language === "ru" ? "Популярно сейчас" : "Trending Now"}
                  </h2>
                  <div className="flex flex-wrap gap-2">
                    {[
                      "Gaming",
                      "ASMR",
                      "Programming",
                      "Art",
                      "Music",
                      "Chatting",
                    ].map((tag) => (
                      <button
                        key={tag}
                        onClick={() => setQuery(tag)}
                        className="px-4 py-2 rounded-full bg-[#18181b] border border-white/5 hover:border-[#D673A9]/50 hover:bg-[#D673A9]/5 text-sm font-bold text-[#a1a1aa] hover:text-[#D673A9] transition-all"
                      >
                        {tag}
                      </button>
                    ))}
                  </div>
                </section>
              </div>

              {/* Right Column: Suggested Content */}
              <div className="lg:col-span-8 space-y-10">
                <section>
                  <div className="flex items-center justify-between mb-6">
                    <h2 className="text-xl font-black text-white">
                      {language === "ru"
                        ? "Популярные категории"
                        : "Top Categories"}
                    </h2>
                    <button
                      className="text-xs font-black uppercase tracking-widest text-[#D673A9] hover:underline"
                      onClick={() => setQuery(" ")}
                    >
                      View All
                    </button>
                  </div>
                  <div className="grid grid-cols-2 sm:grid-cols-4 gap-4">
                    {trendingCategories.map((cat) => (
                      <div
                        key={cat.id}
                        className="group cursor-pointer"
                        onClick={() => setQuery(cat.title)}
                      >
                        <div className="relative aspect-[3/4] rounded-2xl overflow-hidden mb-3 bg-[#18181b] ring-1 ring-white/5 group-hover:ring-[#D673A9]/50 transition-all shadow-xl">
                          <img
                            src={cat.image}
                            alt={cat.title}
                            className="w-full h-full object-cover transition-transform duration-700 group-hover:scale-110"
                          />
                          <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent opacity-60"></div>
                          <div className="absolute top-2 right-2 px-2 py-1 rounded-lg bg-black/50 backdrop-blur-md border border-white/10 text-[10px] font-black text-white">
                            {cat.viewers.split(" ")[0]}
                          </div>
                        </div>
                        <p className="font-bold text-white group-hover:text-[#D673A9] transition-colors truncate">
                          {cat.title}
                        </p>
                        <p className="text-[11px] text-[#55555a] font-bold uppercase tracking-tight">
                          {cat.tags[0]}
                        </p>
                      </div>
                    ))}
                  </div>
                </section>

                <section>
                  <h2 className="text-xl font-black text-white mb-6">
                    {language === "ru"
                      ? "Рекомендуемые каналы"
                      : "Recommended Channels"}
                  </h2>
                  <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    {SIDEBAR_POPULAR.slice(0, 4).map((streamer) => (
                      <div
                        key={streamer.id}
                        className="flex items-center gap-4 p-4 rounded-2xl bg-white/5 border border-white/5 hover:border-[#D673A9]/30 hover:bg-white/10 transition-all cursor-pointer group"
                        onClick={() => onStreamClick(streamer)}
                      >
                        <div className="relative">
                          <img
                            src={streamer.avatar}
                            alt={streamer.name}
                            className="w-16 h-16 rounded-full border-2 border-transparent group-hover:border-[#D673A9] transition-colors"
                          />
                          <div className="absolute -bottom-1 left-1/2 -translate-x-1/2 bg-red-600 text-[8px] font-black uppercase px-1.5 py-0.5 rounded-md border-2 border-[#0e0e10]">
                            Live
                          </div>
                        </div>
                        <div className="flex-1 min-w-0">
                          <p className="font-bold text-white group-hover:text-[#D673A9] transition-colors truncate">
                            {streamer.name}
                          </p>
                          <p className="text-xs text-[#55555a] font-medium truncate">
                            {streamer.game}
                          </p>
                          <p className="text-[11px] text-[#D673A9] font-black mt-1">
                            {streamer.viewers}{" "}
                            {language === "ru" ? "зрителей" : "viewers"}
                          </p>
                        </div>
                      </div>
                    ))}
                  </div>
                </section>
              </div>
            </div>
          ) : (
            <div className="space-y-12 animate-fadeIn">
              {filteredStreamers.length > 0 && (
                <section>
                  <h3 className="text-xs font-black uppercase tracking-[0.2em] text-[#55555a] mb-6 flex items-center gap-2">
                    <User size={14} className="text-[#D673A9]" /> Channels
                  </h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    {filteredStreamers.map((s) => (
                      <div
                        key={s.id}
                        className="flex items-center gap-4 p-4 rounded-3xl bg-[#18181b] border border-white/5 hover:border-[#D673A9]/50 transition-all cursor-pointer group"
                        onClick={() => onStreamClick(s)}
                      >
                        <img
                          src={s.avatar}
                          alt={s.name}
                          className="w-14 h-14 rounded-full border-2 border-transparent group-hover:border-[#D673A9] transition-all"
                        />
                        <div className="flex-1 min-w-0">
                          <p className="font-bold text-white group-hover:text-[#D673A9] transition-colors truncate">
                            {s.name}
                          </p>
                          <p className="text-xs text-[#55555a] font-medium truncate">
                            {s.game}
                          </p>
                          <div className="flex items-center gap-2 mt-1">
                            <div className="w-1.5 h-1.5 rounded-full bg-red-500 animate-pulse"></div>
                            <span className="text-[10px] font-black text-[#D673A9] uppercase">
                              {s.viewers}
                            </span>
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                </section>
              )}

              {filteredCategories.length > 0 && (
                <section>
                  <h3 className="text-xs font-black uppercase tracking-[0.2em] text-[#55555a] mb-6 flex items-center gap-2">
                    <Hash size={14} className="text-blue-500" /> Categories
                  </h3>
                  <div className="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8 gap-4">
                    {filteredCategories.map((cat) => (
                      <div
                        key={cat.id}
                        className="group cursor-pointer"
                        onClick={() => setQuery(cat.title)}
                      >
                        <div className="relative aspect-[3/4] rounded-xl overflow-hidden mb-3 bg-[#18181b] ring-1 ring-white/5 group-hover:ring-[#D673A9]/50 transition-all">
                          <img
                            src={cat.image}
                            alt={cat.title}
                            className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
                          />
                        </div>
                        <p className="font-bold text-sm text-white group-hover:text-[#D673A9] transition-colors truncate leading-tight">
                          {cat.title}
                        </p>
                        <p className="text-[10px] text-[#55555a] font-bold uppercase">
                          {cat.viewers.split(" ")[0]}
                        </p>
                      </div>
                    ))}
                  </div>
                </section>
              )}

              {filteredStreamers.length === 0 &&
                filteredCategories.length === 0 && (
                  <div className="flex flex-col items-center justify-center py-24 text-[#55555a]">
                    <div className="w-24 h-24 rounded-full bg-white/5 flex items-center justify-center mb-8 border border-white/5">
                      <Search size={40} className="opacity-20" />
                    </div>
                    <span className="text-2xl font-black text-white/50 mb-2">
                      No results for "{query}"
                    </span>
                    <p className="text-sm font-medium">
                      Try checking your spelling or use more general keywords.
                    </p>
                  </div>
                )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
