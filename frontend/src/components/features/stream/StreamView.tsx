"use client";

import React, { useState, useEffect, useRef } from "react";
import {
  Heart,
  Share,
  Users,
  MessageSquare,
  Send,
  Smile,
  Gift,
  Trophy,
  ExternalLink,
  Camera,
  Bird,
  Code,
} from "lucide-react";
import { DICT, Language } from "@/lib/i18n";
import { Streamer } from "@/lib/data";

interface StreamViewProps {
  streamer: Streamer;
  language: Language;
}

interface ChatMsg {
  id: number;
  user: string;
  text: string;
  color: string;
  isMod?: boolean;
  isVip?: boolean;
}

const STICKERS = ["🔥", "😂", "😮", "❤️", "👍", "🤡", "👑", "GG"];

export function StreamView({ streamer, language }: StreamViewProps) {
  const t = DICT[language];
  const [messages, setMessages] = useState<ChatMsg[]>([
    {
      id: 1,
      user: "Alex_Pro",
      text: "PogChamp! What a play!",
      color: "#D673A9",
    },
    { id: 2, user: "KillerBee", text: "Is this real life?", color: "#60a5fa" },
    {
      id: 3,
      user: "StreamMod",
      text: "Welcome everyone! Please be respectful.",
      color: "#4ade80",
      isMod: true,
    },
    {
      id: 4,
      user: "ZeldaFan",
      text: "Anyone knows the song name?",
      color: "#fbbf24",
      isVip: true,
    },
  ]);
  const [inputText, setInputText] = useState("");
  const [showStickers, setShowStickers] = useState(false);
  const [bonusPoints, setBonusPoints] = useState(1250);
  const chatEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    chatEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  useEffect(scrollToBottom, [messages]);

  const handleSendMessage = (e?: React.FormEvent) => {
    e?.preventDefault();
    if (!inputText.trim()) return;

    const newMsg: ChatMsg = {
      id: Date.now(),
      user: "Dmitry_X",
      text: inputText,
      color: "#D673A9",
    };

    setMessages([...messages, newMsg]);
    setInputText("");
    setBonusPoints((prev) => prev + 10);
  };

  const addSticker = (sticker: string) => {
    setInputText((prev) => prev + " " + sticker);
    setShowStickers(false);
  };

  return (
    <div className="flex h-full overflow-hidden flex-col">
      <div className="flex flex-1 overflow-hidden">
        {/* Player Area */}
        <div className="flex-1 flex flex-col min-w-0 bg-black overflow-y-auto custom-scrollbar">
          <div className="relative aspect-video bg-black overflow-hidden group shrink-0">
            <div className="absolute inset-0 flex items-center justify-center bg-gradient-to-br from-[#131315] to-[#27272a]">
              <div className="relative w-full h-full flex items-center justify-center">
                <div
                  className="absolute inset-0 opacity-20 pointer-events-none"
                  style={{
                    backgroundImage:
                      "radial-gradient(#D673A9 1px, transparent 1px)",
                    backgroundSize: "20px 20px",
                  }}
                ></div>
                <span className="text-white/10 font-black text-8xl italic select-none tracking-tighter">
                  STREMO PLAYER
                </span>
                <div className="absolute inset-0 flex items-center justify-center">
                  <div className="w-20 h-20 rounded-full bg-[#D673A9]/20 flex items-center justify-center border border-[#D673A9]/50 animate-pulse">
                    <div className="w-0 h-0 border-t-[15px] border-t-transparent border-l-[25px] border-l-[#D673A9] border-b-[15px] border-b-transparent ml-2"></div>
                  </div>
                </div>
              </div>
            </div>

            <div className="absolute top-4 left-4 bg-red-600 text-white px-2 py-0.5 rounded font-bold text-xs uppercase tracking-wider flex items-center gap-1.5 shadow-lg">
              <div className="w-1.5 h-1.5 rounded-full bg-white animate-pulse"></div>
              Live
            </div>

            <div className="absolute bottom-0 left-0 right-0 h-24 bg-gradient-to-t from-black/90 via-black/40 to-transparent opacity-0 group-hover:opacity-100 transition-opacity flex items-center px-8 gap-6">
              <div className="flex items-center gap-4">
                <div className="w-5 h-5 border-2 border-white/80 border-t-transparent rounded-full"></div>
                <div className="w-12 h-1.5 bg-white/20 rounded-full overflow-hidden">
                  <div className="h-full bg-white w-2/3"></div>
                </div>
              </div>
              <div className="flex-1"></div>
              <div className="flex gap-4 items-center">
                <div className="text-white font-bold text-sm">1080p60</div>
                <div className="w-8 h-5 bg-white/20 rounded border border-white/30"></div>
              </div>
            </div>
          </div>

          <div className="p-6 bg-[#18181b]">
            {/* Channel Info */}
            <div className="flex items-start justify-between gap-4 mb-8">
              <div className="flex gap-4">
                {/* eslint-disable-next-line @next/next/no-img-element */}
                <img
                  src={streamer.avatar}
                  alt={streamer.name}
                  className="w-16 h-16 rounded-full border-2 border-[#D673A9] shadow-[0_0_15px_rgba(214,115,169,0.3)]"
                />
                <div className="flex flex-col">
                  <h2 className="text-2xl font-bold text-white line-clamp-1">
                    {streamer.title || `${streamer.name}'s Chill Stream`}
                  </h2>
                  <div className="flex items-center gap-2 mt-1">
                    <span className="text-sm font-black text-[#D673A9] hover:underline cursor-pointer uppercase tracking-wide">
                      {streamer.name}
                    </span>
                    <span className="text-sm text-[#a1a1aa] font-medium">
                      • {streamer.game}
                    </span>
                  </div>
                </div>
              </div>

              <div className="flex items-center gap-3">
                <div className="flex items-center gap-2 px-3 py-2 rounded-xl bg-[#27272a] text-white">
                  <Users size={18} className="text-[#D673A9]" />
                  <span className="text-sm font-bold">{streamer.viewers}</span>
                </div>
                <button className="flex items-center gap-2 px-6 py-2.5 rounded-xl bg-[#D673A9] text-black font-black hover:bg-[#C25B96] transition-all transform active:scale-95 shadow-lg shadow-[#D673A9]/20">
                  <Heart size={18} fill="currentColor" strokeWidth={0} />{" "}
                  {t.follow}
                </button>
                <button className="p-2.5 rounded-xl bg-[#27272a] hover:bg-[#3f3f46] text-white transition-colors border border-white/5">
                  <Share size={20} />
                </button>
              </div>
            </div>

            {/* About & Banners Section */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
              <div className="md:col-span-2 space-y-6">
                <div className="p-6 rounded-2xl bg-[#131315] border border-[#27272a] hover:border-[#D673A9]/30 transition-colors">
                  <h3 className="text-lg font-bold mb-4 flex items-center gap-2">
                    <Trophy size={20} className="text-[#D673A9]" /> {t.about}
                  </h3>
                  {/*Replace on real ddtd in rq #2-#3*/}
                  <p className="text-[#a1a1aa] leading-relaxed text-sm">
                    Welcome to my channel! I'm {streamer.name}, a professional{" "}
                    {streamer.game} player. Don't forget to follow and join our
                    discord for giveaways and community games! Business:
                    contact@{streamer.name.toLowerCase()}.com
                  </p>
                  <div className="flex gap-4 mt-6">
                    <div className="flex flex-col">
                      <span className="text-white font-bold text-lg">1.2M</span>
                      <span className="text-[#a1a1aa] text-xs uppercase font-black">
                        {t.followers}
                      </span>
                    </div>
                    <div className="w-px bg-[#27272a] mx-2"></div>
                    <div className="flex flex-col">
                      <span className="text-white font-bold text-lg">8.5k</span>
                      <span className="text-[#a1a1aa] text-xs uppercase font-black">
                        Subs
                      </span>
                    </div>
                  </div>
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <div className="aspect-[2/1] rounded-2xl bg-gradient-to-br from-[#D673A9]/20 to-[#18181b] border border-[#D673A9]/20 flex items-center justify-center p-6 cursor-pointer hover:scale-[1.02] transition-transform">
                    <div className="text-center">
                      <div className="w-10 h-10 rounded-full bg-[#D673A9] mx-auto mb-3 flex items-center justify-center">
                        <ExternalLink size={20} className="text-black" />
                      </div>
                      <span className="font-bold text-white block">
                        Merch Store
                      </span>
                      <span className="text-xs text-[#D673A9] font-black uppercase">
                        20% OFF NOW
                      </span>
                    </div>
                  </div>
                  <div className="aspect-[2/1] rounded-2xl bg-gradient-to-br from-[#27272a] to-[#131315] border border-white/5 flex items-center justify-center p-6 cursor-pointer hover:scale-[1.02] transition-transform">
                    <div className="text-center">
                      <div className="w-10 h-10 rounded-full bg-[#a1a1aa] mx-auto mb-3 flex items-center justify-center">
                        <Camera size={20} className="text-black" />
                      </div>
                      <span className="font-bold text-white block">
                        Instagram
                      </span>
                      <span className="text-xs text-[#a1a1aa] font-black uppercase">
                        @official_{streamer.name.toLowerCase()}
                      </span>
                    </div>
                  </div>
                </div>
              </div>

              <div className="space-y-4">
                <div className="p-5 rounded-2xl bg-[#27272a]/50 border border-white/5">
                  <h4 className="text-xs font-black text-[#a1a1aa] uppercase mb-4 tracking-widest">
                    {t.links}
                  </h4>
                  <div className="flex flex-col gap-2">
                    <SocialLink
                      icon={<Bird size={16} />}
                      label="Twitter"
                      url="#"
                    />
                    <SocialLink
                      icon={<Camera size={16} />}
                      label="Instagram"
                      url="#"
                    />
                    <SocialLink
                      icon={<Code size={16} />}
                      label="Discord"
                      url="#"
                    />
                  </div>
                </div>

                <div className="p-5 rounded-2xl bg-gradient-to-br from-[#D673A9] to-[#C25B96] text-black">
                  <h4 className="text-sm font-black uppercase mb-1">
                    STREMO PLUS
                  </h4>
                  <p className="text-[10px] font-bold leading-tight mb-3">
                    Subscribe for exclusive emotes and ad-free viewing!
                  </p>
                  <button className="w-full py-2 bg-black text-white rounded-lg text-xs font-black uppercase hover:bg-black/80 transition-colors">
                    Upgrade
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Chat Area */}
        <aside className="w-[340px] hidden lg:flex flex-col border-l border-[#27272a] bg-[#131315] shrink-0">
          <div className="h-14 flex items-center justify-between px-4 border-b border-[#27272a]">
            <h3 className="text-xs font-black uppercase tracking-widest text-[#a1a1aa]">
              Stream Chat
            </h3>
            <div className="flex items-center gap-2 text-[#D673A9]">
              <Trophy size={14} />
              <span className="text-[11px] font-black">{bonusPoints} pts</span>
            </div>
          </div>

          <div className="flex-1 overflow-y-auto p-4 custom-scrollbar space-y-4">
            {messages.map((msg) => (
              <ChatMessage
                key={msg.id}
                user={msg.user}
                text={msg.text}
                color={msg.color}
                isMod={msg.isMod}
                isVip={msg.isVip}
              />
            ))}
            <div ref={chatEndRef} />
          </div>

          <div className="p-4 border-t border-[#27272a] bg-[#18181b]/50">
            <div className="relative mb-3">
              {showStickers && (
                <div className="absolute bottom-full left-0 right-0 mb-2 p-3 bg-[#131315] border border-[#27272a] rounded-2xl shadow-2xl grid grid-cols-4 gap-2 animate-dropdown">
                  {STICKERS.map((s) => (
                    <button
                      key={s}
                      onClick={() => addSticker(s)}
                      className="text-xl hover:scale-125 transition-transform"
                    >
                      {s}
                    </button>
                  ))}
                </div>
              )}

              <form onSubmit={handleSendMessage} className="relative">
                <input
                  type="text"
                  value={inputText}
                  onChange={(e) => setInputText(e.target.value)}
                  placeholder="Send a message"
                  className="w-full bg-[#131315] border border-[#27272a] focus:border-[#D673A9]/50 rounded-xl px-4 py-3 pr-24 text-sm text-white outline-none transition-colors"
                />
                <div className="absolute right-2 top-1/2 -translate-y-1/2 flex items-center gap-1">
                  <button
                    type="button"
                    onClick={() => setShowStickers(!showStickers)}
                    className="p-1.5 text-[#a1a1aa] hover:text-[#D673A9] transition-colors"
                  >
                    <Smile size={18} />
                  </button>
                  <button
                    type="submit"
                    disabled={!inputText.trim()}
                    className="p-1.5 text-[#a1a1aa] hover:text-[#D673A9] transition-colors disabled:opacity-30"
                  >
                    <Send size={18} />
                  </button>
                </div>
              </form>
            </div>

            <div className="flex items-center justify-between px-1">
              <div className="flex gap-2">
                <button className="flex items-center gap-1.5 px-2 py-1 rounded bg-[#27272a] hover:bg-[#3f3f46] text-[#D673A9] transition-colors">
                  <Gift size={14} />
                  <span className="text-[10px] font-black uppercase">Subs</span>
                </button>
                <button className="flex items-center gap-1.5 px-2 py-1 rounded bg-[#27272a] hover:bg-[#3f3f46] text-[#fbbf24] transition-colors">
                  <Trophy size={14} />
                  <span className="text-[10px] font-black uppercase">Bits</span>
                </button>
              </div>
              <div
                className="flex items-center gap-2 group cursor-pointer"
                onClick={() => setBonusPoints((prev) => prev + 50)}
              >
                <div className="w-6 h-6 rounded-lg bg-[#D673A9] flex items-center justify-center text-black font-black text-[10px] group-hover:scale-110 transition-transform">
                  +
                </div>
                <span className="text-[10px] font-black text-[#a1a1aa] uppercase tracking-widest group-hover:text-[#D673A9] transition-colors">
                  {t.chatBonus}
                </span>
              </div>
            </div>
          </div>
        </aside>
      </div>
    </div>
  );
}

function SocialLink({ icon, label, url }: any) {
  return (
    <a
      href={url}
      className="flex items-center justify-between p-3 rounded-xl bg-white/5 hover:bg-white/10 transition-colors group"
    >
      <div className="flex items-center gap-3">
        <span className="text-[#a1a1aa] group-hover:text-white transition-colors">
          {icon}
        </span>
        <span className="text-sm font-bold text-white">{label}</span>
      </div>
      <ExternalLink
        size={14}
        className="text-[#a1a1aa] opacity-0 group-hover:opacity-100 transition-opacity"
      />
    </a>
  );
}

function ChatMessage({ user, text, color, isMod, isVip }: any) {
  return (
    <div className="flex flex-col gap-0.5 leading-snug">
      <div className="flex items-center gap-1.5 flex-wrap">
        {isMod && (
          <div
            className="p-0.5 bg-green-500 rounded text-[8px] text-white"
            title="Moderator"
          >
            <Trophy size={10} fill="currentColor" strokeWidth={0} />
          </div>
        )}
        {isVip && (
          <div
            className="p-0.5 bg-[#fbbf24] rounded text-[8px] text-white"
            title="VIP"
          >
            <Trophy size={10} fill="currentColor" strokeWidth={0} />
          </div>
        )}
        <span
          className="text-[13px] font-black hover:underline cursor-pointer"
          style={{ color }}
        >
          {user}
        </span>
        <span className="text-[13px] text-gray-200">{text}</span>
      </div>
    </div>
  );
}
