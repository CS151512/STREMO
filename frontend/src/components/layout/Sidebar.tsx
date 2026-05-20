"use client";

import React from "react";
import { DICT, Language } from "@/lib/i18n";
import {
  SIDEBAR_FOLLOWING,
  SIDEBAR_POPULAR,
  SIDEBAR_CATEGORIES,
  Streamer,
} from "@/lib/data";
import { Clock } from "lucide-react";

interface SidebarProps {
  language: Language;
  onStreamClick: (streamer: Streamer) => void;
}

export function Sidebar({ language, onStreamClick }: SidebarProps) {
  const t = DICT[language];

  return (
    <aside className="w-[260px] hidden xl:flex flex-col shrink-0 bg-[#131315] border-r border-white/5 overflow-y-auto custom-scrollbar pb-6 z-10">
      <div className="flex flex-col gap-8 px-4 py-6">
        <SidebarSection
          title={t.sidebarFollowing}
          items={SIDEBAR_FOLLOWING}
          btnText={t.showMore}
          onClick={onStreamClick}
          type="streamer"
        />
        <SidebarSection
          title={t.sidebarPopular}
          items={SIDEBAR_POPULAR}
          btnText={t.showMore}
          onClick={onStreamClick}
          type="streamer"
        />
        <SidebarSection
          title={t.sidebarCategories}
          items={SIDEBAR_CATEGORIES}
          btnText={t.showMore}
          type="category"
        />

        <div className="flex flex-col gap-4">
          <h3 className="text-[10px] font-black text-[#55555a] uppercase tracking-[0.2em] px-1 flex items-center gap-2">
            <Clock size={12} /> History
          </h3>
          <div className="flex flex-col gap-3">
            {SIDEBAR_FOLLOWING.slice(0, 3).map((item) => (
              <div
                key={item.id}
                className="flex items-center gap-3 px-1 cursor-pointer group opacity-60 hover:opacity-100 transition-opacity"
                onClick={() => onStreamClick(item)}
              >
                <img
                  src={item.avatar}
                  alt={item.name}
                  className="w-8 h-8 rounded-full grayscale group-hover:grayscale-0 transition-all"
                />
                <div className="flex-1 min-w-0">
                  <p className="text-[13px] font-bold text-white truncate">
                    {item.name}
                  </p>
                  <p className="text-[10px] text-[#55555a] font-bold uppercase truncate">
                    Last seen 2d ago
                  </p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </aside>
  );
}

function SidebarSection({ title, items, btnText, onClick, type }: any) {
  return (
    <div className="flex flex-col gap-4">
      <h3 className="text-[10px] font-black text-[#55555a] uppercase tracking-[0.2em] px-1">
        {title}
      </h3>
      <div className="flex flex-col gap-3">
        {items.map((item: any) => (
          <div
            key={item.id}
            className="flex items-center gap-3 px-1 cursor-pointer group"
            onClick={() => (onClick ? onClick(item) : null)}
          >
            <div className="relative shrink-0">
              <img
                src={type === "streamer" ? item.avatar : item.image}
                alt={type === "streamer" ? item.name : item.title}
                className={`w-8 h-8 object-cover border border-white/5 ${type === "streamer" ? "rounded-full" : "rounded-lg"}`}
              />
              {type === "streamer" && item.isLive && (
                <div className="absolute -bottom-0.5 -right-0.5 w-2.5 h-2.5 bg-red-600 rounded-full border-2 border-[#131315]"></div>
              )}
            </div>
            <div className="flex flex-col flex-1 min-w-0">
              <div className="flex items-center justify-between">
                <span className="text-[13px] font-bold text-white truncate group-hover:text-[#D673A9] transition-colors">
                  {type === "streamer" ? item.name : item.title}
                </span>
                <div className="flex items-center gap-1.5 shrink-0 pl-2">
                  {item.isLive && (
                    <div className="w-1 h-1 rounded-full bg-[#D673A9] animate-pulse"></div>
                  )}
                  <span
                    className={`text-[11px] font-black tabular-nums ${item.isLive ? "text-white" : "text-[#55555a]"}`}
                  >
                    {item.viewers}
                  </span>
                </div>
              </div>
              <span className="text-[11px] text-[#55555a] font-bold uppercase truncate mt-0.5 tracking-tight">
                {type === "streamer" ? item.game || "Offline" : item.desc}
              </span>
            </div>
          </div>
        ))}
      </div>
      <button className="text-[10px] text-[#D673A9] hover:text-[#C25B96] font-black uppercase tracking-widest px-1 w-fit transition-colors">
        {btnText}
      </button>
    </div>
  );
}
