/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React from 'react';
import { CreditCard, Globe, Heart, Wallet, PiggyBank, Briefcase } from 'lucide-react';

interface LedgerIconProps {
  emoji: string;
  name?: string;
  className?: string;
}

export function LedgerIcon({ emoji, name = '', className = 'w-5 h-5' }: LedgerIconProps) {
  const normalizedName = name.toLowerCase();

  // Detect card
  if (emoji === '💳' || normalizedName.includes('卡') || normalizedName.includes('credit')) {
    return <CreditCard className={`${className} text-cyan-600`} />;
  }
  // Detect travel/global
  if (emoji === '✈️' || normalizedName.includes('海淘') || normalizedName.includes('海外') || normalizedName.includes('usd')) {
    return <Globe className={`${className} text-indigo-600`} />;
  }
  // Detect heart/shared
  if (emoji === '💑' || normalizedName.includes('情侣') || normalizedName.includes('老婆') || normalizedName.includes('金库')) {
    return <Heart className={`${className} text-rose-600`} />;
  }
  // Detect business
  if (normalizedName.includes('商务') || normalizedName.includes('工作') || normalizedName.includes('项目')) {
    return <Briefcase className={`${className} text-amber-600`} />;
  }
  // Fallback piggy bank or wallet
  if (emoji === '💰') {
    return <PiggyBank className={`${className} text-emerald-600`} />;
  }

  return <Wallet className={`${className} text-slate-600`} />;
}

export function getLedgerBgColor(emoji: string, name = ''): string {
  const normalizedName = name.toLowerCase();

  if (emoji === '💳' || normalizedName.includes('卡') || normalizedName.includes('credit')) {
    return 'bg-cyan-50 border-cyan-100 text-cyan-700';
  }
  if (emoji === '✈️' || normalizedName.includes('海淘') || normalizedName.includes('海外') || normalizedName.includes('usd')) {
    return 'bg-indigo-50 border-indigo-100 text-indigo-700';
  }
  if (emoji === '💑' || normalizedName.includes('情侣') || normalizedName.includes('老婆') || normalizedName.includes('金库')) {
    return 'bg-rose-50 border-rose-100 text-rose-700';
  }
  if (normalizedName.includes('商务') || normalizedName.includes('工作') || normalizedName.includes('项目')) {
    return 'bg-amber-50 border-amber-100 text-amber-700';
  }
  if (emoji === '💰') {
    return 'bg-emerald-50 border-emerald-100 text-emerald-700';
  }

  return 'bg-slate-50 border-slate-100 text-slate-700';
}
