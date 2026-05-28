/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React, { useState } from 'react';
import { ViewType, User } from '../types';
import { LegerRSLogo } from './LegerRSLogo';
import { 
  Home, 
  BookOpen, 
  BarChart2, 
  Settings, 
  LogOut, 
  Coins, 
  TrendingUp, 
  Wallet,
  Activity,
  Heart,
  Sparkles
} from 'lucide-react';

interface SidebarProps {
  currentView: ViewType;
  setView: (view: ViewType) => void;
  user: User;
  onLogout: () => void;
}

const HUMAN_QUOTES = [
  "理账，是一场与时间的温柔和解。让每份柴米油盐，皆成岁月的温度。",
  "偶尔为小小的快乐与惊喜买单，也是生活里不可或缺的旷野微风。",
  "克制而非吝啬，大方而不虚荣。澄澈的心，带来最充实安稳的长路。",
  "每一笔记录，都是我们在喧嚣尘世里认真爱着生活的珍贵证据。",
  "所谓安全感，不过是钱袋里有沉甸底气，心怀中留一寸温柔闲暇。"
];

export function Sidebar({ currentView, setView, user, onLogout }: SidebarProps) {
  // Navigation links
  const navItems = [
    { view: 'dashboard' as ViewType, label: '账户总览', icon: Home },
    { view: 'ledgers' as ViewType, label: '我的账簿', icon: BookOpen },
    { view: 'reports' as ViewType, label: '财务报表', icon: BarChart2 },
    { view: 'settings' as ViewType, label: '账本设置', icon: Settings },
  ];

  return (
    <nav className="w-[245px] sticky top-0 h-screen bg-white border-r border-slate-200/90 flex flex-col p-5 flex-shrink-0 z-10 transition-all select-none">
      {/* Brand logo */}
      <div className="px-2 py-2 mb-6">
        <LegerRSLogo size="md" />
      </div>

      {/* Primary menu items */}
      <div className="flex flex-col gap-1">
        {navItems.map((item) => {
          const isActive = currentView === item.view || 
            (item.view === 'ledgers' && ['transactions', 'categories', 'budgets'].includes(currentView));
          const IconComponent = item.icon;
          return (
            <button
              key={item.view}
              onClick={() => setView(item.view)}
              className={`flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-xs text-left font-semibold cursor-pointer transition-all border ${
                isActive
                  ? 'bg-indigo-50/70 border-indigo-100/50 text-indigo-700 shadow-3xs font-bold'
                  : 'text-slate-600 hover:text-slate-900 hover:bg-slate-50 border-transparent'
              }`}
            >
              <IconComponent className={`w-4 h-4 transition-colors ${isActive ? 'text-indigo-600' : 'text-slate-400'}`} />
              <span className="tracking-wide">{item.label}</span>
              {item.view === 'ledgers' && (
                <span className={`ml-auto text-[9px] font-bold rounded-full px-2 py-0.5 ${
                  isActive ? 'bg-indigo-200/50 text-indigo-800' : 'bg-slate-100 text-slate-500'
                }`}>
                  3
                </span>
              )}
            </button>
          );
        })}
      </div>

      {/* Sticky User info with option button */}
      <div className="mt-auto pt-4 border-t border-slate-100">
        <div className="flex items-center justify-between mb-4 px-1">
          <div className="flex items-center gap-2.5">
            <div className="w-8.5 h-8.5 rounded-xl bg-slate-50 border border-slate-200 text-slate-700 flex items-center justify-center font-bold text-xs shadow-inner">
              {user.username.substring(0, 1)}
            </div>
            <div>
              <div className="font-bold text-xs text-slate-800 tracking-tight leading-tight">{user.username}</div>
              <div className="text-[10px] font-medium text-slate-400 truncate w-[110px] mt-0.5">{user.email}</div>
            </div>
          </div>
        </div>
        <button
          onClick={onLogout}
          className="w-full flex items-center justify-center gap-2 px-3 py-2.5 border border-slate-100 hover:bg-rose-50 hover:text-rose-600 hover:border-rose-100 text-slate-500 hover:font-bold rounded-xl text-xs font-semibold cursor-pointer transition-all bg-white shadow-3xs"
        >
          <LogOut className="w-4 h-4 text-slate-400 group-hover:text-rose-600" />
          <span>注销退出</span>
        </button>
      </div>
    </nav>
  );
}
