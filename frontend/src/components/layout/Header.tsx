"use client";

import React, { useState } from "react";
import {
  Home,
  Copy,
  User,
  Search,
  Bell,
  MessageCircle,
  Settings,
  Globe,
  LogOut,
  ChevronLeft,
  ChevronRight,
  Check,
} from "lucide-react";
import { DICT, Language } from "@/lib/i18n";
import { SearchOverlay } from "./SearchOverlay";
import { Streamer } from "@/lib/data";

interface HeaderProps {
  activeTab: string;
  setActiveTab: (tab: string) => void;
  language: Language;
  setLanguage: (lang: Language) => void;
  searchQuery: string;
  setSearchQuery: (query: string) => void;
  onStreamClick: (streamer: Streamer) => void;
}

export function Header({
  activeTab,
  setActiveTab,
  language,
  setLanguage,
  searchQuery,
  setSearchQuery,
  onStreamClick,
}: HeaderProps) {
  const [isProfileOpen, setIsProfileOpen] = useState(false);
  const [profileView, setProfileView] = useState<"main" | "lang">("main");
  const [isSearchOpen, setIsSearchOpen] = useState(false);
  const t = DICT[language];

  React.useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "/" && !isSearchOpen) {
        e.preventDefault();
        setIsSearchOpen(true);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isSearchOpen]);

  return (
    <header className="h-20 flex items-center justify-between px-6 shrink-0 bg-background z-50">
      <div className="flex items-center gap-8">
        <div
          className="flex items-center gap-2 cursor-pointer"
          onClick={() => setActiveTab("home")}
        >
          <svg
            viewBox="0 0 100 100"
            className="w-10 h-10 text-pink-primary fill-current"
          >
            <path d="M50 0 L58 12 L72 8 L76 22 L90 25 L88 39 L100 50 L88 61 L90 75 L76 78 L72 92 L58 88 L50 100 L42 88 L28 92 L24 78 L10 75 L12 61 L0 50 L12 39 L10 25 L24 22 L28 8 L42 12 Z" />
          </svg>
          <span className="text-2xl font-black tracking-wider text-white">
            STREMO
          </span>
        </div>

        <nav className="hidden lg:flex items-center gap-3">
          <button
            onClick={() => setActiveTab("home")}
            className={`flex items-center gap-2 h-10 px-4 rounded-full font-bold text-sm transition-all ${activeTab === "home" ? "bg-white text-black" : "text-white hover:bg-white/10"}`}
          >
            <Home
              size={18}
              fill={activeTab === "home" ? "currentColor" : "none"}
              strokeWidth={activeTab === "home" ? 0 : 2}
            />{" "}
            {t.home}
          </button>
          <button
            onClick={() => setActiveTab("browse")}
            className={`flex items-center gap-2 h-10 px-4 rounded-full font-bold text-sm transition-all ${activeTab === "browse" ? "bg-white text-black" : "text-white hover:bg-white/10"}`}
          >
            <Copy size={18} strokeWidth={2} /> {t.browse}
          </button>
          <button
            onClick={() => setActiveTab("following")}
            className={`flex items-center gap-2 h-10 px-4 rounded-full font-bold text-sm transition-all ${activeTab === "following" ? "bg-white text-black" : "text-white hover:bg-white/10"}`}
          >
            <User size={18} strokeWidth={2} /> {t.following}
          </button>
        </nav>
      </div>

      <div className="hidden md:flex flex-1 max-w-lg mx-8">
        <div
          onClick={() => setIsSearchOpen(true)}
          className="flex items-center w-full h-10 px-4 rounded-full bg-[#27272a]/60 border border-transparent hover:border-[#D673A9]/30 transition-all cursor-text group"
        >
          <Search
            size={18}
            className="text-[#a1a1aa] group-hover:text-[#D673A9] transition-colors"
          />
          <div className="flex-1 px-3 text-sm font-medium text-[#a1a1aa]">
            {searchQuery || t.search}
          </div>
          <div className="px-1.5 py-0.5 rounded border border-white/10 text-[10px] font-black text-[#a1a1aa] uppercase tracking-widest">
            /
          </div>
        </div>
      </div>

      <SearchOverlay
        isOpen={isSearchOpen}
        onClose={() => setIsSearchOpen(false)}
        query={searchQuery}
        setQuery={setSearchQuery}
        language={language}
        onStreamClick={(s) => {
          onStreamClick(s);
          setIsSearchOpen(false);
        }}
      />

      <div className="flex items-center gap-4">
        <button className="relative p-2 rounded-full hover:bg-white/10 transition-colors text-white">
          <Bell size={20} fill="currentColor" strokeWidth={0} />
          <div className="absolute top-2 right-2.5 w-1.5 h-1.5 bg-white rounded-full"></div>
        </button>
        <button className="p-2 rounded-full hover:bg-white/10 transition-colors text-white mr-2">
          <MessageCircle size={20} fill="currentColor" strokeWidth={0} />
        </button>

        <button
          onClick={() => setActiveTab("dashboard")}
          className="hidden sm:flex items-center gap-2 h-9 px-4 rounded-full bg-[#D673A9] hover:bg-[#C25B96] text-black font-bold text-sm transition-transform active:scale-95"
        >
          <div className="w-2 h-2 bg-black rounded-full"></div> {t.goLive}
        </button>

        <div className="relative">
          {/* eslint-disable-next-line @next/next/no-img-element */}
          <img
            src="https://i.pravatar.cc/150?u=me"
            alt="Profile"
            onClick={() => {
              setIsProfileOpen(!isProfileOpen);
              setProfileView("main");
            }}
            className={`w-9 h-9 rounded-full cursor-pointer bg-white border-2 transition-colors ${isProfileOpen ? "border-[#D673A9]" : "border-transparent hover:border-[#D673A9]/50"}`}
          />

          {isProfileOpen && (
            <>
              <div
                className="fixed inset-0 z-40"
                onClick={() => setIsProfileOpen(false)}
              ></div>
              <div className="absolute right-0 top-12 w-64 bg-[#18181b] border border-[#27272a] rounded-2xl shadow-[0_15px_40px_rgba(0,0,0,0.6)] overflow-hidden z-50 flex flex-col animate-dropdown">
                <div className="p-4 border-b border-[#27272a] flex items-center gap-3">
                  {/* eslint-disable-next-line @next/next/no-img-element */}
                  <img
                    src="https://i.pravatar.cc/150?u=me"
                    alt="Profile"
                    className="w-10 h-10 rounded-full"
                  />
                  <div>
                    <p className="font-bold text-sm text-white">Dmitry_X</p>
                    <div className="flex items-center gap-1.5 mt-0.5">
                      <div className="w-1.5 h-1.5 rounded-full bg-green-500"></div>
                      <p className="text-xs text-[#a1a1aa] font-medium">
                        {t.online}
                      </p>
                    </div>
                  </div>
                </div>

                {profileView === "main" ? (
                  <div className="p-2 flex flex-col">
                    <ProfileMenuItem
                      icon={<User size={18} />}
                      label={t.profile}
                    />
                    <ProfileMenuItem
                      icon={<Settings size={18} />}
                      label={t.settings}
                    />
                    <ProfileMenuItem
                      icon={<Globe size={18} />}
                      label={t.language}
                      value={language === "ru" ? "Русский" : "English"}
                      onClick={() => setProfileView("lang")}
                      showArrow
                    />
                    <div className="h-px bg-[#27272a] my-1 mx-2" />
                    <ProfileMenuItem
                      icon={<LogOut size={18} />}
                      label={t.logout}
                      hoverClass="hover:bg-red-500/10 hover:text-red-500"
                    />
                  </div>
                ) : (
                  <div className="p-2 flex flex-col">
                    <button
                      onClick={() => setProfileView("main")}
                      className="flex items-center gap-2 px-3 py-2.5 text-sm font-bold text-[#a1a1aa] hover:text-white rounded-xl hover:bg-white/5 mb-1 transition-colors"
                    >
                      <ChevronLeft size={16} /> {t.back}
                    </button>
                    <LanguageOption
                      label="English"
                      code="en"
                      active={language === "en"}
                      onSelect={(lang) => {
                        setLanguage(lang as Language);
                        setProfileView("main");
                      }}
                    />
                    <LanguageOption
                      label="Русский"
                      code="ru"
                      active={language === "ru"}
                      onSelect={(lang) => {
                        setLanguage(lang as Language);
                        setProfileView("main");
                      }}
                    />
                  </div>
                )}
              </div>
            </>
          )}
        </div>
      </div>
    </header>
  );
}

function ProfileMenuItem({
  icon,
  label,
  value,
  onClick,
  showArrow,
  hoverClass = "hover:bg-white/5 hover:text-white",
}: any) {
  return (
    <div
      onClick={onClick}
      className={`flex items-center justify-between px-3 py-2.5 rounded-xl cursor-pointer text-[#a1a1aa] transition-colors ${hoverClass}`}
    >
      <div className="flex items-center gap-3">
        {icon}
        <span className="text-sm font-bold">{label}</span>
      </div>
      <div className="flex items-center gap-2">
        {value && (
          <span className="text-xs font-medium bg-[#27272a] px-2 py-0.5 rounded text-gray-300">
            {value}
          </span>
        )}
        {showArrow && <ChevronRight size={16} />}
      </div>
    </div>
  );
}

function LanguageOption({ label, code, active, onSelect }: any) {
  return (
    <div
      onClick={() => onSelect(code)}
      className="flex items-center justify-between px-3 py-2.5 rounded-xl cursor-pointer text-[#a1a1aa] hover:bg-white/5 hover:text-white transition-colors"
    >
      <span className={`text-sm font-bold ${active ? "text-white" : ""}`}>
        {label}
      </span>
      {active && <Check size={16} className="text-[#D673A9]" strokeWidth={3} />}
    </div>
  );
}
