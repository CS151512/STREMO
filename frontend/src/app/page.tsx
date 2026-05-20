"use client";

import React, { useState, useMemo } from 'react';
import { Language } from '@/lib/i18n';
import { Streamer } from '@/lib/data';
import { Header } from '@/components/layout/Header';
import { Sidebar } from '@/components/layout/Sidebar';
import { HeroBanner } from '@/components/features/home/HeroBanner';
import { CategoryGrid } from '@/components/features/home/CategoryGrid';
import { CookieBanner } from '@/components/ui/CookieBanner';
import { StreamView } from '@/components/features/stream/StreamView';
import { BrowseView } from '@/components/features/browse/BrowseView';
import { FollowingView } from '@/components/features/following/FollowingView';
import { SearchView } from '@/components/features/search/SearchView';
import { DashboardView } from '@/components/features/dashboard/DashboardView';
import { Footer } from '@/components/layout/Footer';

type Tab = 'home' | 'browse' | 'following' | 'stream' | 'search' | 'dashboard';

export default function Home() {
  const [activeTab, setActiveTab] = useState<Tab>('home');
  const [language, setLanguage] = useState<Language>('ru'); // Default to Russian
  const [selectedStreamer, setSelectedStreamer] = useState<Streamer | null>(null);
  const [searchQuery, setSearchQuery] = useState('');

  const handleStreamClick = (streamer: Streamer) => {
    setSelectedStreamer(streamer);
    setActiveTab('stream');
    setSearchQuery(''); // Clear search when opening stream
  };

  const currentTab = useMemo(() => {
    if (searchQuery.length > 0) return 'search';
    return activeTab;
  }, [searchQuery, activeTab]);

  const renderContent = () => {
    switch (currentTab) {
      case 'stream':
        return selectedStreamer ? (
          <StreamView streamer={selectedStreamer} language={language} />
        ) : (
          <div className="flex-1 p-8 text-center text-[#a1a1aa]">Выберите стрим для просмотра</div>
        );
      case 'browse':
        return <BrowseView language={language} />;
      case 'following':
        return <FollowingView language={language} onStreamClick={handleStreamClick} />;
      case 'dashboard':
        return <DashboardView language={language} />;
      case 'search':
        return <SearchView query={searchQuery} language={language} onStreamClick={handleStreamClick} />;
      case 'home':
      default:
        return (
          <div className="max-w-[1600px] mx-auto p-8">
            <HeroBanner language={language} onStreamClick={handleStreamClick} />
            <CategoryGrid language={language} />
          </div>
        );
    }
  };

  return (
    <div className="flex flex-col h-screen w-screen bg-background text-[#f4f4f5] font-sans overflow-hidden selection:bg-pink-primary/30 relative">
      
      <Header 
        activeTab={activeTab} 
        setActiveTab={setActiveTab} 
        language={language} 
        setLanguage={setLanguage} 
        searchQuery={searchQuery}
        setSearchQuery={setSearchQuery}
        onStreamClick={handleStreamClick}
      />

      <div className="flex flex-1 overflow-hidden relative">
        <Sidebar language={language} onStreamClick={handleStreamClick} />

        <main className="flex-1 overflow-y-auto custom-scrollbar bg-card rounded-tl-2xl relative z-0 flex flex-col">
          <div className="flex-1">
            {renderContent()}
          </div>
          {currentTab !== 'stream' && <Footer language={language} />}
        </main>
        
        <CookieBanner language={language} />
      </div>
    </div>
  );
}
