"use client";

import React, { useState } from "react";
import { Play, Heart, Share } from "lucide-react";
import { DICT, Language } from "@/lib/i18n";
import { RECOMMENDATIONS, Streamer } from "@/lib/data";

interface HeroBannerProps {
  language: Language;
  onStreamClick: (streamer: any) => void;
}

export function HeroBanner({ language, onStreamClick }: HeroBannerProps) {
  const t = DICT[language];
  const [recIndex, setRecIndex] = useState(0);

  const handleWatchNow = () => {
    const currentRec = RECOMMENDATIONS[recIndex];
    onStreamClick({
      id: currentRec.id,
      name: currentRec.channel,
      game: currentRec.game,
      viewers: currentRec.viewers,
      avatar: currentRec.avatar,
      isLive: true,
      title: currentRec.title,
    });
  };

  return (
    <div className="relative w-full rounded-2xl mb-12 flex flex-col lg:flex-row overflow-hidden min-h-[440px]">
      <div className="absolute inset-0 z-0">
        {/* eslint-disable-next-line @next/next/no-img-element */}
        <img
          src="https://picsum.photos/seed/witcher3_bg/1200/600"
          alt="Hero Background"
          className="w-full h-full object-cover opacity-60"
        />
        <div className="absolute inset-0 bg-gradient-to-r from-[#18181b] via-[#18181b]/80 to-transparent"></div>
        <div className="absolute inset-0 bg-gradient-to-t from-[#18181b] via-transparent to-transparent"></div>
      </div>

      <div className="relative z-10 flex-1 py-10 px-4 lg:px-8 flex flex-col justify-center max-w-xl">
        <div className="mb-6">
          <h2 className="text-3xl font-serif italic font-bold tracking-widest text-gray-300 drop-shadow-lg">
            THE WITCHER 3
          </h2>
          <h3 className="text-xl font-serif tracking-widest text-red-600 drop-shadow-md">
            WILD HUNT
          </h3>
        </div>

        <div className="flex flex-wrap gap-2 mb-4">
          {["RPG", "Adventure Game", "Action", "Open World"].map((tag) => (
            <span
              key={tag}
              className="px-3 py-1 rounded-full text-[11px] font-bold bg-[#27272a] text-gray-300"
            >
              {tag}
            </span>
          ))}
        </div>

        <h1 className="text-[2.5rem] font-bold mb-4 leading-tight">
          The Witcher 3: Wild Hunt
        </h1>
        <p className="text-sm text-[#a1a1aa] mb-8 leading-relaxed max-w-md">
          Explore a dark-fantasy open world as monster slayer Geralt of Rivia in
          The Witcher 3: Wild Hunt, one of the most acclaimed role-playing games
          of all time.
        </p>

        <div className="flex flex-col gap-3">
          <button
            onClick={handleWatchNow}
            className="w-full sm:w-[85%] flex items-center justify-center gap-2 h-12 rounded-xl bg-[#D673A9] hover:bg-[#C25B96] text-black font-bold text-[15px] transition-transform active:scale-95"
          >
            <Play size={18} fill="currentColor" strokeWidth={0} /> {t.watchNow}
          </button>
          <div className="flex gap-3 w-full sm:w-[85%]">
            <button className="flex-1 flex items-center justify-center gap-2 h-12 rounded-xl bg-[#27272a] hover:bg-[#3f3f46] text-white font-bold text-sm transition-transform active:scale-95">
              <Heart size={18} /> {t.follow}
            </button>
            <button className="flex-1 flex items-center justify-center gap-2 h-12 rounded-xl bg-[#27272a] hover:bg-[#3f3f46] text-white font-bold text-sm transition-transform active:scale-95">
              <Share size={18} /> {t.share}
            </button>
          </div>
        </div>
      </div>

      {/* Recommendation Slider */}
      <div className="relative z-10 lg:w-[580px] xl:w-[620px] py-10 pl-4 pr-0 flex flex-col justify-center overflow-hidden">
        <div className="flex items-center justify-between mb-5 pr-4 relative z-40">
          <h3 className="font-bold text-[17px]">{t.recStreamers}</h3>
          <div className="flex gap-2 items-center bg-[#131315]/80 border border-white/5 rounded-full px-4 py-1.5 backdrop-blur-sm shadow-lg">
            <button
              onClick={() =>
                setRecIndex(
                  (prev) =>
                    (prev - 1 + RECOMMENDATIONS.length) %
                    RECOMMENDATIONS.length,
                )
              }
              className="w-5 h-5 flex items-center justify-center text-gray-400 hover:text-white font-medium transition-colors"
            >
              &lt;
            </button>
            {RECOMMENDATIONS.map((_, idx) => (
              <button
                key={idx}
                onClick={() => setRecIndex(idx)}
                className={`w-1.5 h-1.5 rounded-full transition-all duration-300 mx-0.5 ${idx === recIndex ? "bg-white scale-150" : "bg-gray-500 hover:bg-gray-400"}`}
              />
            ))}
            <button
              onClick={() =>
                setRecIndex((prev) => (prev + 1) % RECOMMENDATIONS.length)
              }
              className="w-5 h-5 flex items-center justify-center text-gray-400 hover:text-white font-medium transition-colors"
            >
              &gt;
            </button>
          </div>
        </div>

        {/* Slider Track */}
        <div className="relative w-full h-[260px] sm:h-[300px]">
          <div
            className="flex gap-5 transition-transform duration-500 ease-[cubic-bezier(0.25,1,0.5,1)] w-max [--card-width:280px] sm:[--card-width:320px]"
            style={{
              transform: `translateX(calc(-${recIndex} * (var(--card-width) + 1.25rem)))`,
            }}
          >
            {RECOMMENDATIONS.map((stream, idx) => {
              const isActive = idx === recIndex;
              return (
                <div
                  key={stream.id}
                  onClick={() => {
                    setRecIndex(idx);
                    if (isActive) {
                      onStreamClick({
                        id: stream.id,
                        name: stream.channel,
                        game: stream.game,
                        viewers: stream.viewers,
                        avatar: stream.avatar,
                        isLive: true,
                        title: stream.title,
                      });
                    }
                  }}
                  className={`w-[var(--card-width)] shrink-0 flex flex-col gap-3 transition-opacity duration-500 cursor-pointer ${isActive ? "opacity-100" : "opacity-50 hover:opacity-80"}`}
                >
                  <div className="relative aspect-video rounded-2xl overflow-hidden bg-[#18181b] shadow-[0_15px_40px_rgba(0,0,0,0.7)] border-[1.5px] border-white/20 backdrop-blur-xl group">
                    <div className="absolute inset-0 bg-gradient-to-br from-white/10 to-transparent z-10 pointer-events-none opacity-60 group-hover:opacity-100 transition-opacity"></div>
                    {/* eslint-disable-next-line @next/next/no-img-element */}
                    <img
                      src={stream.image}
                      className="w-full h-full object-cover"
                      alt=""
                    />

                    <div className="absolute top-3 left-3 bg-[#f03e3e] text-white text-[11px] font-bold px-2 py-0.5 rounded-md z-20 shadow-md">
                      LIVE
                    </div>
                    <div className="absolute bottom-3 left-3 bg-[#131315]/90 backdrop-blur-md text-white text-[11px] font-bold px-2.5 py-1.5 rounded-lg shadow-sm z-20 border border-white/5">
                      {stream.viewers}
                    </div>
                  </div>

                  <div className="flex flex-col px-1">
                    <h4 className="text-[14px] font-bold leading-snug mb-2.5 line-clamp-2 drop-shadow-md">
                      {stream.title}
                    </h4>
                    <div className="flex items-center gap-3">
                      {/* eslint-disable-next-line @next/next/no-img-element */}
                      <img
                        src={stream.avatar}
                        className="w-7 h-7 rounded-full object-cover shadow-sm border border-white/10"
                        alt=""
                      />
                      <div className="flex flex-col">
                        <span className="text-[12px] font-bold leading-none mb-1">
                          {stream.channel}
                        </span>
                        <span className="text-[11px] text-[#a1a1aa] leading-none">
                          {stream.game}
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      </div>
    </div>
  );
}
