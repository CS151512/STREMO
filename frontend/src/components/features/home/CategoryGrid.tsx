"use client";

import React from "react";
import { DICT, Language } from "@/lib/i18n";
import { BOTTOM_CATEGORIES } from "@/lib/data";

interface CategoryGridProps {
  language: Language;
}

export function CategoryGrid({ language }: CategoryGridProps) {
  const t = DICT[language];

  return (
    <>
      <div className="flex items-center justify-between mb-8">
        <h2 className="text-xl font-black text-white">{t.categoriesLike}</h2>
        <button className="text-[10px] font-black uppercase tracking-[0.2em] text-[#D673A9] hover:text-[#C25B96] transition-colors">
          {t.viewAll}
        </button>
      </div>

      <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-5 gap-6 pb-12">
        {BOTTOM_CATEGORIES.map((category) => (
          <div key={category.id} className="flex flex-col group cursor-pointer">
            <div className="relative aspect-[3/4] rounded-2xl overflow-hidden mb-4 bg-[#27272a] shadow-xl transition-all group-hover:translate-y-[-4px]">
              {/* eslint-disable-next-line @next/next/no-img-element */}
              <img
                src={category.image}
                alt={category.title}
                className="w-full h-full object-cover transition-transform duration-700 group-hover:scale-110"
              />
              <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent opacity-60"></div>
              <div className="absolute bottom-4 left-0 right-0 text-center">
                <span className="font-black text-xl tracking-[0.3em] uppercase text-white/40 group-hover:text-white/80 transition-colors drop-shadow-lg">
                  STREMO
                </span>
              </div>
            </div>

            <h3 className="font-bold text-[15px] text-white group-hover:text-[#D673A9] transition-colors truncate">
              {category.title}
            </h3>
            <p className="text-[11px] text-[#55555a] font-black uppercase tracking-tight mt-0.5">
              {category.viewers}
            </p>

            <div className="flex flex-wrap gap-1.5 mt-3">
              {category.tags.slice(0, 2).map((tag) => (
                <span
                  key={tag}
                  className="px-2 py-0.5 rounded-md text-[9px] font-black uppercase bg-white/5 text-[#55555a] border border-white/5"
                >
                  {tag}
                </span>
              ))}
            </div>
          </div>
        ))}
      </div>
    </>
  );
}
