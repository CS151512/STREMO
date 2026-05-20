"use client";

import React from "react";
import { DICT, Language } from "@/lib/i18n";
import { SIDEBAR_FOLLOWING, Streamer } from "@/lib/data";

interface FollowingViewProps {
  language: Language;
  onStreamClick: (streamer: Streamer) => void;
}

export function FollowingView({ language, onStreamClick }: FollowingViewProps) {
  const t = DICT[language];
  const liveStreamers = SIDEBAR_FOLLOWING.filter((s) => s.isLive);
  const offlineStreamers = SIDEBAR_FOLLOWING.filter((s) => !s.isLive);

  return (
    <div className="max-w-[1600px] mx-auto p-8">
      <h1 className="text-4xl font-black mb-10">{t.following}</h1>

      <section className="mb-12">
        <h2 className="text-xl font-bold mb-6 flex items-center gap-3">
          Live Channels
          <span className="px-2 py-0.5 bg-red-600 text-white text-[10px] rounded uppercase">
            Live
          </span>
        </h2>
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6">
          {liveStreamers.map((streamer) => (
            <StreamCard
              key={streamer.id}
              streamer={streamer}
              onClick={() => onStreamClick(streamer)}
            />
          ))}
        </div>
      </section>

      <section>
        <h2 className="text-xl font-bold mb-6 text-[#a1a1aa]">
          Offline Channels
        </h2>
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-8 gap-4 opacity-60">
          {offlineStreamers.map((streamer) => (
            <div
              key={streamer.id}
              className="flex flex-col items-center text-center group cursor-pointer"
              onClick={() => onStreamClick(streamer)}
            >
              {/* eslint-disable-next-line @next/next/no-img-element */}
              <img
                src={streamer.avatar}
                alt={streamer.name}
                className="w-20 h-20 rounded-full mb-3 grayscale group-hover:grayscale-0 transition-all"
              />
              <span className="text-sm font-bold text-white group-hover:text-[#D673A9]">
                {streamer.name}
              </span>
            </div>
          ))}
        </div>
      </section>
    </div>
  );
}

function StreamCard({ streamer, onClick }: any) {
  return (
    <div className="flex flex-col group cursor-pointer" onClick={onClick}>
      <div className="relative aspect-video rounded-xl overflow-hidden mb-3 bg-[#27272a]">
        {/* eslint-disable-next-line @next/next/no-img-element */}
        <img
          src={`https://picsum.photos/seed/${streamer.name}/600/338`}
          alt={streamer.name}
          className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
        />
        <div className="absolute top-3 left-3 bg-red-600 text-white text-[10px] font-bold px-2 py-0.5 rounded uppercase">
          Live
        </div>
        <div className="absolute bottom-3 left-3 bg-black/60 backdrop-blur-md text-white text-[10px] font-bold px-2 py-1 rounded">
          {streamer.viewers} viewers
        </div>
      </div>
      <div className="flex gap-3">
        {/* eslint-disable-next-line @next/next/no-img-element */}
        <img
          src={streamer.avatar}
          alt={streamer.name}
          className="w-10 h-10 rounded-full shrink-0"
        />
        <div className="flex flex-col min-w-0">
          <h3 className="font-bold text-[14px] text-white line-clamp-1 group-hover:text-[#D673A9] transition-colors">
            Chill Stream with {streamer.name}
          </h3>
          <p className="text-[12px] text-[#a1a1aa] mt-0.5">{streamer.name}</p>
          <p className="text-[11px] text-[#a1a1aa]">{streamer.game}</p>
        </div>
      </div>
    </div>
  );
}
