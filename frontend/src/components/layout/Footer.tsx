"use client";

import React from "react";
import { DICT, Language } from "@/lib/i18n";
import { Bird, Camera, Code, Play } from "lucide-react";

interface FooterProps {
  language: Language;
}

export function Footer({ language }: FooterProps) {
  const t = DICT[language];

  return (
    <footer className="bg-[#131315] border-t border-[#27272a] py-12 px-8 mt-auto">
      <div className="max-w-[1600px] mx-auto grid grid-cols-1 md:grid-cols-4 gap-12">
        <div className="col-span-1 md:col-span-1">
          <div className="flex items-center gap-2 mb-6">
            <svg
              viewBox="0 0 100 100"
              className="w-8 h-8 text-[#D673A9] fill-current"
            >
              <path d="M50 0 L58 12 L72 8 L76 22 L90 25 L88 39 L100 50 L88 61 L90 75 L76 78 L72 92 L58 88 L50 100 L42 88 L28 92 L24 78 L10 75 L12 61 L0 50 L12 39 L10 25 L24 22 L28 8 L42 12 Z" />
            </svg>
            <span className="text-xl font-black tracking-wider text-white">
              STREMO
            </span>
          </div>
          <p className="text-sm text-[#a1a1aa] leading-relaxed mb-6">
            {t.description}
          </p>
          <div className="flex gap-4">
            <SocialIcon icon={<Bird size={18} />} />
            <SocialIcon icon={<Camera size={18} />} />
            <SocialIcon icon={<Play size={18} />} />
            <SocialIcon icon={<Code size={18} />} />
          </div>
        </div>

        <div>
          <h4 className="text-white font-black text-sm uppercase mb-6 tracking-widest">
            Platform
          </h4>
          <ul className="space-y-4 text-sm text-[#a1a1aa]">
            <li className="hover:text-[#D673A9] cursor-pointer transition-colors">
              About Us
            </li>
            <li className="hover:text-[#D673A9] cursor-pointer transition-colors">
              Careers
            </li>
            <li className="hover:text-[#D673A9] cursor-pointer transition-colors">
              Press
            </li>
            <li className="hover:text-[#D673A9] cursor-pointer transition-colors">
              Blog
            </li>
          </ul>
        </div>

        <div>
          <h4 className="text-white font-black text-sm uppercase mb-6 tracking-widest">
            Support
          </h4>
          <ul className="space-y-4 text-sm text-[#a1a1aa]">
            <li className="hover:text-[#D673A9] cursor-pointer transition-colors">
              Help Center
            </li>
            <li className="hover:text-[#D673A9] cursor-pointer transition-colors">
              Safety Center
            </li>
            <li className="hover:text-[#D673A9] cursor-pointer transition-colors">
              Community Guidelines
            </li>
            <li className="hover:text-[#D673A9] cursor-pointer transition-colors">
              Ad Choices
            </li>
          </ul>
        </div>

        <div>
          <h4 className="text-white font-black text-sm uppercase mb-6 tracking-widest">
            Legal
          </h4>
          <ul className="space-y-4 text-sm text-[#a1a1aa]">
            <li className="hover:text-[#D673A9] cursor-pointer transition-colors">
              Privacy Policy
            </li>
            <li className="hover:text-[#D673A9] cursor-pointer transition-colors">
              Terms of Service
            </li>
            <li className="hover:text-[#D673A9] cursor-pointer transition-colors">
              Cookie Policy
            </li>
            <li className="hover:text-[#D673A9] cursor-pointer transition-colors">
              GDPR
            </li>
          </ul>
        </div>
      </div>

      <div className="max-w-[1600px] mx-auto mt-12 pt-8 border-t border-[#27272a] flex flex-col md:flex-row justify-between items-center gap-4">
        <p className="text-xs text-[#a1a1aa] font-medium">{t.copyright}</p>
        <div className="flex gap-6 text-[10px] font-black uppercase text-[#a1a1aa] tracking-widest">
          <span className="hover:text-white cursor-pointer transition-colors">
            English
          </span>
          <span className="text-[#D673A9]">Русский</span>
          <span className="hover:text-white cursor-pointer transition-colors">
            Deutsch
          </span>
          <span className="hover:text-white cursor-pointer transition-colors">
            Français
          </span>
        </div>
      </div>
    </footer>
  );
}

function SocialIcon({ icon }: any) {
  return (
    <div className="w-10 h-10 rounded-full bg-[#27272a] flex items-center justify-center text-[#a1a1aa] hover:bg-[#D673A9] hover:text-black transition-all cursor-pointer">
      {icon}
    </div>
  );
}
