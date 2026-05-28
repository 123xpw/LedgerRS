/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import { Ledger, Transaction, Category, Budget, User } from './types';

export const initialUser: User = {
  username: '胡小明',
  email: 'hu.xiaoming@example.com',
  defaultCurrency: 'CNY',
  timezone: 'Asia/Shanghai',
};

export const initialLedgers: Ledger[] = [
  { id: '1', name: '日常活期手账', emoji: '💳', currency: 'CNY', balance: 5240.50 },
  { id: '2', name: '海外海淘卡', emoji: '✈️', currency: 'USD', balance: 420.00 },
  { id: '3', name: '老婆共用小金库', emoji: '💑', currency: 'CNY', balance: 18400.00 },
];

export const initialCategories: Category[] = [
  // Expense
  { id: 'e1', name: '日常餐饮', type: 'expense', color: '#ef4444' },
  { id: 'e2', name: '数码极客', type: 'expense', color: '#8b5cf6' },
  { id: 'e3', name: '房租房贷', type: 'expense', color: '#f59e0b' },
  { id: 'e4', name: '交通出行', type: 'expense', color: '#10b981' },
  { id: 'e5', name: '休闲娱乐', type: 'expense', color: '#ec4899' },
  
  // Income
  { id: 'i1', name: '劳动薪资', type: 'income', color: '#16a34a' },
  { id: 'i2', name: '理财投资', type: 'income', color: '#0ea5e9' },
  { id: 'i3', name: '副业副刊', type: 'income', color: '#84cc16' },
];

export const initialTransactions: Transaction[] = [
  { id: 't1', date: '2026-05-28', type: 'expense', amount: 35.50, currency: 'CNY', category: '日常餐饮', note: '麦当劳安格斯牛堡午餐' },
  { id: 't2', date: '2026-05-27', type: 'expense', amount: 15.00, currency: 'CNY', category: '交通出行', note: '地铁乘车扣款' },
  { id: 't3', date: '2026-05-25', type: 'income', amount: 12500.00, currency: 'CNY', category: '劳动薪资', note: '5月份常规月度薪金' },
  { id: 't4', date: '2026-05-24', type: 'expense', amount: 289.00, currency: 'CNY', category: '数码极客', note: '美商海盗船电竞鼠标' },
  { id: 't5', date: '2026-05-20', type: 'expense', amount: 3200.00, currency: 'CNY', category: '房租房贷', note: '月度整租房租扣缴' },
  { id: 't6', date: '2026-05-18', type: 'income', amount: 450.00, currency: 'CNY', category: '副业副刊', note: '定制Logo设计接单分成' },
];

export const initialBudgets: Budget[] = [
  { id: 'b1', name: '本月餐饮限额', category: '日常餐饮', period: 'monthly', limitAmount: 1200.00, spentAmount: 520.50 },
  { id: 'b2', name: '电子数码发烧友', category: '数码极客', period: 'monthly', limitAmount: 1500.00, spentAmount: 289.00 },
  { id: 'b3', name: '总包控制', period: 'monthly', limitAmount: 6000.00, spentAmount: 4024.50 },
];

// Presets for new users to quickly solve the "less categories, too much blank space" issue
export const CATEGORY_PRESETS = [
  {
    name: '自由职业设计师',
    categories: [
      { name: '日常餐饮', type: 'expense', color: '#ef4444' },
      { name: '交通出行', type: 'expense', color: '#10b981' },
      { name: '云服务费/SaaS订阅', type: 'expense', color: '#6366f1' },
      { name: '硬件外设', type: 'expense', color: '#8b5cf6' },
      { name: '咖啡馆续命', type: 'expense', color: '#ec4899' },
      { name: '项目合同款', type: 'income', color: '#16a34a' },
      { name: '设计分成/版税', type: 'income', color: '#0ea5e9' },
    ]
  },
  {
    name: '大学穷游一族',
    categories: [
      { name: '食堂伙食', type: 'expense', color: '#f59e0b' },
      { name: '火车/客运大巴', type: 'expense', color: '#10b981' },
      { name: '青旅/特价酒店', type: 'expense', color: '#06b6d4' },
      { name: '景区门票', type: 'expense', color: '#8b5cf6' },
      { name: '防晒及洗护', type: 'expense', color: '#ec4899' },
      { name: '月度生活费', type: 'income', color: '#16a34a' },
      { name: '兼职家教/勤工助学', type: 'income', color: '#eab308' },
    ]
  },
  {
    name: '幸福三口之家',
    categories: [
      { name: '超市买菜', type: 'expense', color: '#10b981' },
      { name: '孩子学费/兴趣班', type: 'expense', color: '#8b5cf6' },
      { name: '水电燃气物业费', type: 'expense', color: '#64748b' },
      { name: '房贷/车贷月供', type: 'expense', color: '#ef4444' },
      { name: '医疗与商业保险', type: 'expense', color: '#06b6d4' },
      { name: '家庭主薪资', type: 'income', color: '#16a34a' },
      { name: '年终绩效/奖金', type: 'income', color: '#0ea5e9' },
    ]
  }
];
