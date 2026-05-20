"use client";

import React, { useState } from "react";
import { Cookie } from "lucide-react";
import { DICT, Language } from "@/lib/i18n";

interface CookieBannerProps {
  language: Language;
}

export function CookieBanner({ language }: CookieBannerProps) {
  const [showCookieBanner, setShowCookieBanner] = useState(true);
  const [isCookieClosing, setIsCookieClosing] = useState(false);

  const t = DICT[language];

  const handleCloseCookie = () => {
    setIsCookieClosing(true);
    setTimeout(() => setShowCookieBanner(false), 400);
  };

  if (!showCookieBanner) return null;

  return (
    <div
      className={`fixed bottom-6 left-1/2 w-[calc(100%-3rem)] max-w-4xl bg-[#18181b]/95 backdrop-blur-xl border border-[#27272a] px-5 py-4 rounded-2xl shadow-[0_20px_50px_rgba(0,0,0,0.5)] z-[100] flex flex-col sm:flex-row items-start sm:items-center justify-between gap-5 ${isCookieClosing ? "animate-cookie-out" : "animate-cookie-in"}`}
    >
      <div className="flex items-start sm:items-center gap-4 text-sm text-[#a1a1aa]">
        <div className="p-2.5 bg-[#27272a]/50 rounded-full shrink-0">
          <Cookie className="text-[#D673A9]" size={24} />
        </div>
        <p className="leading-relaxed text-[13px] sm:text-sm font-medium">
          {t.cookieText}
        </p>
      </div>
      <div className="flex gap-3 w-full sm:w-auto shrink-0">
        <button
          onClick={handleCloseCookie}
          className="flex-1 sm:flex-none px-6 py-2.5 text-sm font-bold text-white bg-[#27272a] hover:bg-[#3f3f46] rounded-xl transition-transform active:scale-95"
        >
          {t.decline}
        </button>
        <button
          onClick={handleCloseCookie}
          className="flex-1 sm:flex-none px-6 py-2.5 text-sm font-bold bg-[#D673A9] hover:bg-[#C25B96] text-black rounded-xl transition-transform active:scale-95"
        >
          {t.accept}
        </button>
      </div>
    </div>
  );
}
