"use client";

import React, { useState } from "react";
import {
  Settings,
  Mic,
  Video,
  Activity,
  MessageSquare,
  Plus,
  Edit,
  Users,
  BarChart,
  Layers,
  Radio,
  Camera,
  Volume2,
  Monitor,
  Zap,
  Shield,
  Image,
  Cpu,
  MoreVertical,
  Send,
  Heart,
  Play,
} from "lucide-react";
import { DICT, Language } from "@/lib/i18n";

interface DashboardViewProps {
  language: Language;
}

export function DashboardView({ language }: DashboardViewProps) {
  const t = DICT[language];
  const [activeMenu, setActiveMenu] = useState("Settings");
  const [isLive, setIsLive] = useState(false);

  return (
    <div className="flex h-[calc(100vh-80px)] bg-[#0e0e10] overflow-hidden animate-fadeIn">
      {/* Dashboard Sidebar */}
      <aside className="w-64 bg-[#131315] border-r border-white/5 flex flex-col shrink-0">
        <div className="p-4 space-y-1">
          <DashboardMenuItem
            icon={<Zap size={18} />}
            label="Quick Setup"
            active={activeMenu === "Quick Setup"}
            onClick={() => setActiveMenu("Quick Setup")}
          />
          <DashboardMenuItem
            icon={<Radio size={18} />}
            label="Stream"
            active={activeMenu === "Stream"}
            onClick={() => setActiveMenu("Stream")}
          />

          <div className="pt-4 pb-2">
            <p className="px-4 text-[10px] font-black uppercase tracking-widest text-[#55555a]">
              Stream Controls
            </p>
          </div>

          <DashboardMenuItem
            icon={<Layers size={18} />}
            label="Scenes & Overlays"
            active={activeMenu === "Scenes"}
            onClick={() => setActiveMenu("Scenes")}
          />
          <DashboardMenuItem
            icon={<MessageSquare size={18} />}
            label="Live Chat"
            active={activeMenu === "Chat"}
            onClick={() => setActiveMenu("Chat")}
          />
          <DashboardMenuItem
            icon={<Users size={18} />}
            label="Guest Management"
            active={activeMenu === "Guests"}
            onClick={() => setActiveMenu("Guests")}
          />
          <DashboardMenuItem
            icon={<Image size={18} />}
            label="Media Library"
            active={activeMenu === "Media"}
            onClick={() => setActiveMenu("Media")}
          />
          <DashboardMenuItem
            icon={<Cpu size={18} />}
            label="AI Features"
            active={activeMenu === "AI"}
            onClick={() => setActiveMenu("AI")}
          />
          <DashboardMenuItem
            icon={<Settings size={18} />}
            label="Settings"
            active={activeMenu === "Settings"}
            onClick={() => setActiveMenu("Settings")}
          />
        </div>
      </aside>

      {/* Main Content Area */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {/* Dashboard Header */}
        <header className="h-16 bg-[#131315]/50 border-b border-white/5 flex items-center justify-between px-6 shrink-0 backdrop-blur-md">
          <div className="flex items-center gap-8">
            <div className="flex flex-col">
              <span className="text-[10px] font-black uppercase tracking-widest text-[#55555a]">
                Session
              </span>
              <span className="text-sm font-bold text-white tabular-nums">
                {isLive ? "00:06:24" : "00:00:00"}
              </span>
            </div>
            <div className="flex flex-col">
              <span className="text-[10px] font-black uppercase tracking-widest text-[#55555a]">
                Viewers
              </span>
              <span className="text-sm font-bold text-white tabular-nums">
                {isLive ? "135" : "0"}
              </span>
            </div>
            <div className="flex flex-col">
              <span className="text-[10px] font-black uppercase tracking-widest text-[#55555a]">
                Followers
              </span>
              <span className="text-sm font-bold text-white tabular-nums">
                678
              </span>
            </div>
          </div>

          <div className="flex items-center gap-3">
            <button className="p-2 rounded-xl bg-white/5 hover:bg-white/10 text-[#a1a1aa] hover:text-white transition-all">
              <Mic size={20} />
            </button>
            <button className="p-2 rounded-xl bg-white/5 hover:bg-white/10 text-[#a1a1aa] hover:text-white transition-all">
              <Camera size={20} />
            </button>
            <button
              onClick={() => setIsLive(!isLive)}
              className={`px-6 py-2 rounded-xl font-black text-xs uppercase tracking-widest transition-all active:scale-95 shadow-lg ${isLive ? "bg-red-500 hover:bg-red-600 text-white" : "bg-[#D673A9] hover:bg-[#C25B96] text-black"}`}
            >
              {isLive ? "End Stream" : "Go Live"}
            </button>
          </div>
        </header>

        {/* Dynamic Content */}
        <main className="flex-1 overflow-y-auto custom-scrollbar p-6 bg-[#0e0e10]">
          {isLive ? (
            <div className="grid grid-cols-12 gap-6 h-full min-h-[600px]">
              {/* Left: Stream Preview & Actions */}
              <div className="col-span-12 lg:col-span-8 space-y-6">
                <section className="bg-[#131315] rounded-3xl border border-white/5 overflow-hidden shadow-2xl">
                  <div className="px-6 py-4 border-b border-white/5 flex items-center justify-between">
                    <h3 className="text-xs font-black uppercase tracking-widest text-[#a1a1aa]">
                      Stream Preview
                    </h3>
                    <div className="flex items-center gap-2">
                      <span className="w-2 h-2 rounded-full bg-red-500 animate-pulse"></span>
                      <span className="text-[10px] font-black uppercase text-red-500 tracking-wider">
                        Live
                      </span>
                    </div>
                  </div>
                  <div className="aspect-video bg-black relative flex items-center justify-center group">
                    <div
                      className="absolute inset-0 opacity-10"
                      style={{
                        backgroundImage:
                          "radial-gradient(#fff 1px, transparent 1px)",
                        backgroundSize: "20px 20px",
                      }}
                    ></div>
                    <div className="text-center">
                      <div className="w-16 h-16 rounded-full bg-white/5 flex items-center justify-center mx-auto mb-4 border border-white/10">
                        <Play
                          size={32}
                          className="text-white fill-white ml-1"
                        />
                      </div>
                      <p className="text-xs font-black uppercase tracking-widest text-[#55555a]">
                        Live Camera Feed
                      </p>
                      <p className="text-[10px] text-[#333336] mt-1">
                        Built-in Camera
                      </p>
                    </div>
                  </div>
                </section>

                <div className="grid grid-cols-2 gap-6">
                  <section className="bg-[#131315] p-6 rounded-3xl border border-white/5">
                    <h3 className="text-xs font-black uppercase tracking-widest text-[#a1a1aa] mb-6">
                      Quick Actions
                    </h3>
                    <div className="grid grid-cols-3 gap-3">
                      <QuickActionButton
                        icon={<Edit size={20} />}
                        label="Edit Info"
                      />
                      <QuickActionButton
                        icon={<BarChart size={20} />}
                        label="Manage Poll"
                      />
                      <QuickActionButton icon={<Plus size={20} />} label="" />
                    </div>
                  </section>
                  <section className="bg-[#131315] p-6 rounded-3xl border border-white/5">
                    <h3 className="text-xs font-black uppercase tracking-widest text-[#a1a1aa] mb-6">
                      Activity Feed
                    </h3>
                    <div className="space-y-4">
                      <ActivityItem
                        name="Ace"
                        action="Followed you"
                        time="12 minutes ago"
                      />
                      <ActivityItem
                        name="Charmer"
                        action="Followed you"
                        time="16 minutes ago"
                      />
                      <ActivityItem
                        name="joker2"
                        action="Followed you"
                        time="29 minutes ago"
                      />
                    </div>
                  </section>
                </div>
              </div>

              {/* Right: Chat */}
              <div className="col-span-12 lg:col-span-4 flex flex-col bg-[#131315] rounded-3xl border border-white/5 overflow-hidden shadow-2xl">
                <div className="px-6 py-4 border-b border-white/5">
                  <h3 className="text-xs font-black uppercase tracking-widest text-[#a1a1aa]">
                    Live Chat
                  </h3>
                </div>
                <div className="flex-1 overflow-y-auto p-4 space-y-4 custom-scrollbar">
                  <ChatBubble
                    name="JognD"
                    text="Good morning everyone! Following this live 👋"
                    color="#D673A9"
                  />
                  <ChatBubble
                    name="AnnaK"
                    text="Do we know when the press conference starts?"
                    color="#60a5fa"
                  />
                  <ChatBubble
                    name="MikeNY"
                    text="Love that GETTR is doing live streams now 🔥"
                    color="#4ade80"
                  />
                  <ChatBubble
                    name="sarah"
                    text="Great coverage, thank you DailyNews!"
                    color="#fbbf24"
                  />
                </div>
                <div className="p-4 border-t border-white/5">
                  <div className="relative">
                    <input
                      type="text"
                      placeholder="Send message"
                      className="w-full bg-[#18181b] rounded-2xl px-4 py-3 text-sm border border-white/5 focus:border-[#D673A9]/50 outline-none transition-all pr-12"
                    />
                    <button className="absolute right-2 top-1/2 -translate-y-1/2 w-8 h-8 rounded-xl bg-[#D673A9] flex items-center justify-center text-black">
                      <Send size={16} />
                    </button>
                  </div>
                </div>
              </div>
            </div>
          ) : (
            <div className="max-w-4xl space-y-8 animate-slideUp">
              <h1 className="text-2xl font-black text-white">{activeMenu}</h1>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
                {/* Audio Controls */}
                <section className="bg-[#131315] p-6 rounded-3xl border border-white/5 shadow-xl">
                  <div className="flex items-center gap-2 mb-8">
                    <Volume2 size={18} className="text-[#D673A9]" />
                    <h3 className="text-xs font-black uppercase tracking-widest text-[#a1a1aa]">
                      Audio Controls
                    </h3>
                  </div>
                  <div className="space-y-6">
                    <ControlSlider label="Speaker Volume" value="25%" />
                    <ControlSlider label="Microphone Volume" value="25%" />
                  </div>
                </section>

                {/* Stream Quality */}
                <section className="bg-[#131315] p-6 rounded-3xl border border-white/5 shadow-xl">
                  <div className="flex items-center gap-2 mb-8">
                    <Monitor size={18} className="text-[#D673A9]" />
                    <h3 className="text-xs font-black uppercase tracking-widest text-[#a1a1aa]">
                      Stream Quality
                    </h3>
                  </div>
                  <div className="space-y-6">
                    <div className="space-y-2">
                      <p className="text-[10px] font-black uppercase tracking-widest text-[#55555a]">
                        Quality
                      </p>
                      <div className="flex items-center justify-between p-3 rounded-xl bg-[#18181b] border border-white/5 text-sm font-bold text-white">
                        1080p (Full HD)
                        <MoreVertical size={16} className="text-[#55555a]" />
                      </div>
                    </div>
                    <div className="space-y-2">
                      <p className="text-[10px] font-black uppercase tracking-widest text-[#55555a]">
                        Bitrate
                      </p>
                      <div className="flex items-center gap-3">
                        <span className="text-xs font-bold text-white">
                          2233 kbps
                        </span>
                        <div className="flex-1 h-1 bg-white/5 rounded-full overflow-hidden">
                          <div className="h-full bg-green-500 w-1/2"></div>
                        </div>
                      </div>
                    </div>
                  </div>
                </section>

                {/* Camera Settings */}
                <section className="bg-[#131315] p-6 rounded-3xl border border-white/5 shadow-xl">
                  <div className="flex items-center gap-2 mb-8">
                    <Camera size={18} className="text-[#D673A9]" />
                    <h3 className="text-xs font-black uppercase tracking-widest text-[#a1a1aa]">
                      Camera Settings
                    </h3>
                  </div>
                  <div className="space-y-6">
                    <ControlSlider label="Brightness" value="25%" />
                    <ControlSlider label="Contrast" value="25%" />
                    <ControlSlider label="Saturation" value="25%" />
                  </div>
                </section>

                {/* Stream Preview (Settings) */}
                <section className="bg-[#131315] rounded-3xl border border-white/5 overflow-hidden shadow-xl">
                  <div className="px-6 py-4 border-b border-white/5">
                    <h3 className="text-xs font-black uppercase tracking-widest text-[#a1a1aa]">
                      Stream Preview
                    </h3>
                  </div>
                  <div className="aspect-square bg-black flex items-center justify-center relative group">
                    <div
                      className="absolute inset-0 opacity-5"
                      style={{
                        backgroundImage:
                          "radial-gradient(#fff 1px, transparent 1px)",
                        backgroundSize: "20px 20px",
                      }}
                    ></div>
                    <div className="text-center">
                      <Play size={24} className="text-[#27272a] mx-auto mb-2" />
                      <p className="text-[10px] font-black uppercase tracking-widest text-[#27272a]">
                        Preview Off
                      </p>
                    </div>
                  </div>
                </section>
              </div>
            </div>
          )}
        </main>
      </div>
    </div>
  );
}

function DashboardMenuItem({ icon, label, active, onClick }: any) {
  return (
    <button
      onClick={onClick}
      className={`w-full flex items-center gap-3 px-4 py-3 rounded-2xl text-sm font-bold transition-all ${active ? "bg-[#D673A9] text-black shadow-lg shadow-[#D673A9]/20 scale-[1.02]" : "text-[#a1a1aa] hover:bg-white/5 hover:text-white"}`}
    >
      {icon}
      {label}
    </button>
  );
}

function ControlSlider({ label, value }: any) {
  return (
    <div className="space-y-3">
      <div className="flex items-center justify-between">
        <p className="text-[10px] font-black uppercase tracking-widest text-[#55555a]">
          {label}
        </p>
        <span className="text-[10px] font-black text-white">{value}</span>
      </div>
      <div className="relative h-1 bg-white/5 rounded-full">
        <div className="absolute top-0 left-0 h-full bg-[#D673A9] rounded-full w-1/4"></div>
        <div className="absolute top-1/2 left-1/4 -translate-y-1/2 w-4 h-4 bg-white rounded-full shadow-lg border-2 border-[#D673A9]"></div>
      </div>
    </div>
  );
}

function QuickActionButton({ icon, label }: any) {
  return (
    <button className="aspect-square rounded-2xl bg-[#18181b] border border-white/5 hover:border-[#D673A9]/50 flex flex-col items-center justify-center gap-2 transition-all group active:scale-95">
      <span className="text-[#55555a] group-hover:text-[#D673A9] transition-colors">
        {icon}
      </span>
      {label && (
        <span className="text-[9px] font-black uppercase tracking-tighter text-[#55555a] group-hover:text-white transition-colors">
          {label}
        </span>
      )}
    </button>
  );
}

function ActivityItem({ name, action, time }: any) {
  return (
    <div className="flex items-center gap-3">
      <div className="w-8 h-8 rounded-full bg-[#D673A9]/10 flex items-center justify-center">
        <Heart size={14} className="text-[#D673A9] fill-[#D673A9]" />
      </div>
      <div className="flex-1 min-w-0">
        <p className="text-xs text-[#a1a1aa] truncate">
          <span className="font-bold text-white">{name}</span> {action}
        </p>
        <p className="text-[10px] text-[#55555a] font-bold uppercase">{time}</p>
      </div>
    </div>
  );
}

function ChatBubble({ name, text, color }: any) {
  return (
    <div className="flex flex-col gap-1">
      <div className="flex items-center gap-2">
        <span className="text-xs font-black" style={{ color }}>
          {name}:
        </span>
        <span className="text-xs text-white leading-tight">{text}</span>
      </div>
    </div>
  );
}
