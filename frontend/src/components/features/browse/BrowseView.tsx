"use client";

import React from "react";
import { DICT, Language } from "@/lib/i18n";
import { BOTTOM_CATEGORIES } from "@/lib/data";

interface BrowseViewProps {
  language: Language;
}

export function BrowseView({ language }: BrowseViewProps) {
  const t = DICT[language];

  return (
    <div className="max-w-[1600px] mx-auto p-8">
      <h1 className="text-4xl font-black mb-8">{t.browse}</h1>

      <div className="flex items-center gap-6 mb-10 border-b border-[#27272a]">
        <button className="pb-4 text-sm font-bold text-[#D673A9] border-b-2 border-[#D673A9]">
          Categories
        </button>
        <button className="pb-4 text-sm font-bold text-[#a1a1aa] hover:text-white transition-colors">
          Live Channels
        </button>
      </div>

      <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-6">
        {BOTTOM_CATEGORIES.map((category) => (
          <div key={category.id} className="flex flex-col group cursor-pointer">
            <div className="relative aspect-[3/4] rounded-xl overflow-hidden mb-3 bg-[#27272a]">
              {/* eslint-disable-next-line @next/next/no-img-element */}
              <img
                src={category.image}
                alt={category.title}
                className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
              />
            </div>
            <h3 className="font-bold text-[15px] mb-0.5 group-hover:text-[#D673A9] transition-colors line-clamp-1">
              {category.title}
            </h3>
            <p className="text-[11px] text-[#a1a1aa] font-medium">
              {category.viewers}
            </p>
            <div className="flex flex-wrap gap-1.5 mt-2">
              {category.tags.slice(0, 2).map((tag) => (
                <span
                  key={tag}
                  className="px-2 py-0.5 rounded-md text-[10px] font-bold bg-[#27272a] text-gray-400"
                >
                  {tag}
                </span>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
