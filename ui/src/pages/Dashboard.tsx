/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React, { useState } from 'react';
import { Ledger, Transaction, Category, Budget } from '../types';
import { CATEGORY_PRESETS } from '../mockData';
import { LedgerIcon, getLedgerBgColor } from '../components/LedgerIcon';
import {
  Sparkles,
  TrendingUp,
  Wallet,
  PlusCircle,
  CheckCircle,
  Award,
  Zap,
  Clock,
  Coins,
  LayoutGrid
} from 'lucide-react';

interface DashboardProps {
  ledgers: Ledger[];
  transactions: Transaction[];
  categories: Category[];
  budgets: Budget[];
  onImportCategories: (cats: Array<{ name: string; type: string; color: string }>) => Promise<void>;
  addTransaction: (tx: Omit<Transaction, 'id'>) => Promise<void>;
  setSelectedLedgerId: (id: string) => void;
  setView: (view: 'transactions' | 'categories' | 'budgets' | 'ledgers' | 'dashboard') => void;
}

export function Dashboard({
  ledgers,
  transactions,
  categories,
  budgets,
  onImportCategories,
  addTransaction,
  setSelectedLedgerId,
  setView
}: DashboardProps) {
  const [quickAmount, setQuickAmount] = useState('');
  const [quickCategory, setQuickCategory] = useState('');
  const [quickNote, setQuickNote] = useState('');
  const [quickLedger, setQuickLedger] = useState(ledgers[0]?.id || '');
  const [showSuccessToast, setShowSuccessToast] = useState(false);
  const [quickError, setQuickError] = useState('');
  const [selectedTemplateIdx, setSelectedTemplateIdx] = useState<number | null>(null);
  const [importMsg, setImportMsg] = useState('');

  const getGreeting = () => {
    const hr = new Date().getHours();
    if (hr < 5) return '深夜好';
    if (hr < 12) return '清晨好';
    if (hr < 18) return '午后好';
    return '傍晚好';
  };

  const totalAsset = ledgers.reduce((acc, l) => {
    const bal = l.balance;
    return acc + (l.currency === 'USD' ? bal * 7.2 : bal);
  }, 0);

  const thisMonthIncome = ledgers.reduce((acc, l) => acc + (l.thisMonthIncome || 0) * (l.currency === 'USD' ? 7.2 : 1), 0);
  const thisMonthExpense = ledgers.reduce((acc, l) => acc + (l.thisMonthExpense || 0) * (l.currency === 'USD' ? 7.2 : 1), 0);

  const handleQuickAdd = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!quickAmount || isNaN(parseFloat(quickAmount))) return;

    const chosenCat = quickCategory || (categories.find(c => c.type === 'expense')?.name || '日常餐饮');
    const ledgerId = quickLedger || ledgers[0]?.id;
    const ledger = ledgers.find(l => l.id === ledgerId);

    setQuickError('');
    try {
      await addTransaction({
        date: new Date().toISOString().split('T')[0],
        type: 'expense',
        amount: parseFloat(parseFloat(quickAmount).toFixed(2)),
        currency: ledger?.currency || 'CNY',
        category: chosenCat,
        note: quickNote || '快速记账'
      });
      setQuickAmount('');
      setQuickNote('');
      setShowSuccessToast(true);
      setTimeout(() => setShowSuccessToast(false), 2500);
    } catch (err) {
      setQuickError(err instanceof Error ? err.message : '记账失败');
    }
  };

  const handleImportTemplate = async (idx: number) => {
    const preset = CATEGORY_PRESETS[idx];
    const existing = new Set(categories.map(c => c.name.toLowerCase()));
    const toCreate = preset.categories.filter(c => !existing.has(c.name.toLowerCase()));

    if (toCreate.length === 0) {
      setSelectedTemplateIdx(idx);
      setImportMsg(`【${preset.name}】分类已全部存在！`);
      setTimeout(() => setImportMsg(''), 2000);
      return;
    }

    try {
      await onImportCategories(toCreate.map(c => ({ name: c.name, type: c.type, color: c.color })));
      setSelectedTemplateIdx(idx);
      setImportMsg(`【${preset.name}】模板加载成功！`);
      setTimeout(() => setImportMsg(''), 2500);
    } catch (err) {
      setImportMsg(err instanceof Error ? err.message : '导入失败');
      setTimeout(() => setImportMsg(''), 2500);
    }
  };

  return (
    <div className="space-y-6">
      {/* Header banner */}
      <div className="bg-white border border-slate-200/90 rounded-2xl p-6 relative overflow-hidden shadow-3xs">
        <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 relative z-10">
          <div className="space-y-1">
            <span className="text-[10px] text-indigo-600 font-extrabold uppercase tracking-widest block">
              📊 LegerRS · 万能多币处理单元
            </span>
            <h2 className="text-lg md:text-xl font-extrabold text-slate-900 tracking-tight mt-1">
              {getGreeting()}。
            </h2>
            <p className="text-xs text-slate-500 font-medium leading-relaxed max-w-xl">
              各本账册已妥善同步，当前外汇资产自动以本地实时比价折算结算。
            </p>
          </div>
          <button
            onClick={() => setView('ledgers')}
            className="px-4 py-2.5 text-xs font-bold bg-white border border-slate-200 hover:border-indigo-500 hover:text-indigo-600 text-slate-700 rounded-xl cursor-pointer transition-all flex items-center gap-1.5 shadow-3xs"
          >
            <LayoutGrid className="w-4 h-4 text-slate-400" />
            <span>进入记账本详情</span>
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Left: main content */}
        <div className="lg:col-span-2 space-y-6">
          {/* Stats */}
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
            <div className="bg-white border border-slate-200 rounded-2xl p-5 hover:border-slate-350 transition-all shadow-xs">
              <div className="space-y-2">
                <div className="text-xs text-slate-400 font-bold uppercase tracking-wider flex items-center gap-1.5">
                  <Coins className="w-3.5 h-3.5 text-indigo-550" />
                  <span>当前合并折算资产</span>
                </div>
                <div className="text-2xl font-extrabold text-slate-900 tracking-tight font-mono">
                  ¥ {totalAsset.toLocaleString('zh-CN', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                </div>
                <div className="text-[10px] text-slate-400 font-medium">旗下所有账户（含外币）自动折合估值</div>
              </div>
            </div>

            <div className="bg-white border border-slate-200 rounded-2xl p-5 hover:border-slate-350 transition-all shadow-xs">
              <div className="space-y-2">
                <div className="text-xs text-slate-400 font-bold uppercase tracking-wider flex items-center gap-1.5">
                  <span className="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
                  <span>本期流水主营开源</span>
                </div>
                <div className="text-2xl font-extrabold text-emerald-600 tracking-tight font-mono">
                  + ¥ {thisMonthIncome.toLocaleString('zh-CN', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                </div>
                <div className="text-[10px] text-slate-400 font-medium">本统计结算周期内录入的总正向流水额</div>
              </div>
            </div>

            <div className="bg-white border border-slate-200 rounded-2xl p-5 hover:border-slate-350 transition-all shadow-xs">
              <div className="space-y-2">
                <div className="text-xs text-slate-400 font-bold uppercase tracking-wider flex items-center gap-1.5">
                  <span className="w-1.5 h-1.5 rounded-full bg-rose-500"></span>
                  <span>本期流水日常支出</span>
                </div>
                <div className="text-2xl font-extrabold text-rose-600 tracking-tight font-mono">
                  - ¥ {thisMonthExpense.toLocaleString('zh-CN', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                </div>
                <div className="text-[10px] text-slate-400 font-medium">本统计结算周期内录入的总日常负向流水额</div>
              </div>
            </div>
          </div>

          {/* Category presets */}
          <div className="bg-slate-50 border border-slate-200 rounded-2xl p-5 relative overflow-hidden shadow-3xs">
            <div className="flex items-center gap-2 mb-2">
              <Sparkles className="w-4.5 h-4.5 text-indigo-600" />
              <h3 className="font-extrabold text-xs tracking-wider uppercase text-slate-800">一键快速启用财务分类包</h3>
            </div>
            <p className="text-slate-500 text-xs mb-4">
              当前您已启用 <strong className="font-extrabold text-indigo-600">{categories.length} 个</strong> 账目品类。您可以根据当前理财需求，一键加载预置的标准科目。
            </p>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
              {CATEGORY_PRESETS.map((preset, idx) => {
                const isSelected = selectedTemplateIdx === idx;
                return (
                  <button
                    key={preset.name}
                    onClick={() => handleImportTemplate(idx)}
                    className={`p-4 rounded-xl border text-left cursor-pointer transition-all ${
                      isSelected
                        ? 'bg-indigo-600 border-indigo-700 text-white shadow-sm'
                        : 'bg-white border-slate-200/90 hover:border-slate-350 text-slate-700'
                    }`}
                  >
                    <div className="font-bold text-xs mb-1 flex justify-between items-center">
                      <span className="tracking-wide">{preset.name}</span>
                      {isSelected ? <CheckCircle className="w-4 h-4 text-white" /> : <Award className="w-4 h-4 text-slate-400" />}
                    </div>
                    <p className={`text-[10px] leading-relaxed ${isSelected ? 'text-indigo-100/90' : 'text-slate-400'}`}>
                      含 {preset.categories.filter(c => c.type === 'expense').length} 个支出标签，{preset.categories.filter(c => c.type === 'income').length} 个收入标签。
                    </p>
                  </button>
                );
              })}
            </div>

            {importMsg && (
              <div className="mt-3.5 p-2.5 bg-emerald-50 rounded-xl text-xs text-emerald-800 font-bold text-center border border-emerald-200/40">
                🎉 {importMsg}
              </div>
            )}
          </div>

          {/* Ledger cards */}
          <div className="space-y-3">
            <div className="flex justify-between items-center">
              <h3 className="text-xs font-bold text-slate-400 uppercase tracking-wider flex items-center gap-1.5">
                <Wallet className="w-4 h-4 text-slate-400" />
                <span>我的独立功能账簿 ({ledgers.length})</span>
              </h3>
              <button
                onClick={() => setView('ledgers')}
                className="text-xs text-indigo-600 font-bold hover:text-indigo-800 transition-colors bg-none border-none cursor-pointer"
              >
                整理账簿资产 →
              </button>
            </div>

            <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
              {ledgers.map((ledger) => (
                <div
                  key={ledger.id}
                  onClick={() => { setSelectedLedgerId(ledger.id); setView('transactions'); }}
                  className="bg-white border border-slate-250/50 rounded-2xl p-5 cursor-pointer hover:border-slate-350 transition-all flex flex-col justify-between"
                >
                  <div className="w-full flex flex-col justify-between h-full">
                    <div>
                      <div className="flex justify-between items-start mb-4">
                        <div className={`w-11 h-11 rounded-xl flex items-center justify-center border font-bold ${getLedgerBgColor(ledger.emoji, ledger.name)} shadow-3xs`}>
                          <LedgerIcon emoji={ledger.emoji} name={ledger.name} className="w-5 h-5" />
                        </div>
                        <span className="text-[10px] font-bold text-slate-500 bg-slate-50 border border-slate-200/60 rounded-md px-2 py-0.5">
                          {ledger.currency}
                        </span>
                      </div>
                      <h4 className="font-bold text-xs text-slate-800">{ledger.name}</h4>
                    </div>

                    <div className="mt-3">
                      <span className="text-[10px] text-slate-400 block font-bold uppercase tracking-wider">账面活期资产</span>
                      <span className="text-lg font-extrabold text-slate-900 tracking-tight block font-mono">
                        {ledger.currency === 'USD' ? '$' : '¥'} {ledger.balance.toLocaleString('zh-CN', { minimumFractionDigits: 2 })}
                      </span>
                    </div>

                    <div className="mt-4 pt-3 border-t border-slate-100 flex justify-between items-center text-[10px] text-slate-400 font-semibold uppercase tracking-wider">
                      <span>快捷入口</span>
                      <div className="flex gap-2 text-indigo-600 font-bold">
                        <span>明细</span>
                        <span className="text-slate-200">|</span>
                        <span>标签</span>
                        <span className="text-slate-200">|</span>
                        <span>预算</span>
                      </div>
                    </div>
                  </div>
                </div>
              ))}

              {ledgers.length === 0 && (
                <div className="col-span-3 bg-white border border-slate-200 rounded-2xl p-8 text-center text-slate-400">
                  <p className="font-semibold text-slate-700 mb-1">还没有账本</p>
                  <button onClick={() => setView('ledgers')} className="text-xs text-indigo-600 font-bold cursor-pointer border-none bg-transparent">去创建第一个账本 →</button>
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Right: quick actions */}
        <div className="space-y-6">
          {/* Quick add */}
          <div className="bg-white border border-slate-200 rounded-2xl p-5 relative overflow-hidden">
            <h3 className="text-xs font-bold text-slate-400 uppercase tracking-wide border-b border-slate-100 pb-3 mb-4 flex items-center justify-between">
              <span className="flex items-center gap-1.5 text-slate-700">
                <Zap className="w-4 h-4 text-indigo-500 fill-indigo-50" />
                <span>🖋️ 快速记账流水录入</span>
              </span>
              <span className="text-[9px] font-extrabold text-indigo-700 bg-indigo-50 border border-indigo-150/30 px-2 py-0.5 rounded-lg uppercase tracking-wider">快捷提录</span>
            </h3>

            {showSuccessToast && (
              <div className="mb-4 p-2.5 bg-emerald-50 border border-emerald-200 text-emerald-800 rounded-xl text-xs font-bold flex items-center gap-1.5 justify-center">
                <span>✓</span> 流水登记成功，对应账本余额已扣减！
              </div>
            )}
            {quickError && (
              <div className="mb-4 p-2.5 bg-rose-50 border border-rose-200 text-rose-800 rounded-xl text-xs font-bold">
                ⚠ {quickError}
              </div>
            )}

            <form onSubmit={handleQuickAdd} className="space-y-4">
              <div className="space-y-1">
                <label className="block text-[10.5px] font-bold text-slate-500">选择入帐本</label>
                <select
                  value={quickLedger}
                  onChange={(e) => setQuickLedger(e.target.value)}
                  className="w-full p-2.5 text-xs border border-slate-200 rounded-xl outline-none focus:border-indigo-500 focus:ring-4 focus:ring-indigo-100 bg-slate-50 font-semibold"
                >
                  {ledgers.map(l => (
                    <option key={l.id} value={l.id}>{l.name} ({l.currency})</option>
                  ))}
                </select>
              </div>

              <div className="space-y-1">
                <label className="block text-[10.5px] font-bold text-slate-500">归属支出分类</label>
                <select
                  value={quickCategory}
                  onChange={(e) => setQuickCategory(e.target.value)}
                  className="w-full p-2.5 text-xs border border-slate-200 rounded-xl outline-none focus:border-indigo-500 focus:ring-4 focus:ring-indigo-100 bg-slate-50 font-semibold"
                >
                  {categories.filter(c => c.type === 'expense').map(c => (
                    <option key={c.id} value={c.name}>{c.name}</option>
                  ))}
                  {categories.filter(c => c.type === 'expense').length === 0 && (
                    <option value="日常餐饮">日常餐饮</option>
                  )}
                </select>
              </div>

              <div className="space-y-1">
                <label className="block text-[10.5px] font-bold text-slate-500">消费流出本金</label>
                <input
                  type="number"
                  step="0.01"
                  required
                  placeholder="0.00"
                  value={quickAmount}
                  onChange={(e) => setQuickAmount(e.target.value)}
                  className="w-full p-2.5 text-xs border border-slate-200 rounded-xl outline-none focus:border-indigo-500 focus:ring-4 focus:ring-indigo-100 bg-slate-50 font-semibold"
                />
              </div>

              <div className="space-y-1">
                <label className="block text-[10.5px] font-bold text-slate-500">交易摘要 / 备注</label>
                <input
                  type="text"
                  placeholder="如: 外卖或写字楼午餐"
                  value={quickNote}
                  onChange={(e) => setQuickNote(e.target.value)}
                  className="w-full p-2.5 text-xs border border-slate-200 rounded-xl outline-none focus:border-indigo-500 focus:ring-4 focus:ring-indigo-100 bg-slate-50 font-semibold"
                />
              </div>

              <button
                type="submit"
                className="w-full bg-indigo-600 hover:bg-indigo-700 hover:scale-[1.01] text-white font-bold py-2.5 rounded-xl text-xs cursor-pointer transition-all flex items-center justify-center gap-1.5 shadow shadow-indigo-100/70"
              >
                <PlusCircle className="w-4 h-4" />
                <span>登记此笔支出</span>
              </button>
            </form>
          </div>

          {/* Budget tracker */}
          <div className="bg-white border border-slate-200 rounded-2xl p-5 shadow-3xs">
            <h3 className="text-xs font-bold text-slate-400 tracking-wide border-b border-slate-100 pb-3 mb-4 flex justify-between items-center">
              <span className="flex items-center gap-1.5 text-slate-700">
                <Clock className="w-4 h-4 text-slate-400" />
                <span>月度防超支限制水位</span>
              </span>
              <button
                onClick={() => setView('budgets')}
                className="text-xs font-bold text-indigo-600 hover:text-indigo-800 transition-colors bg-none border-none cursor-pointer"
              >
                配置限额
              </button>
            </h3>

            <div className="space-y-4">
              {budgets.length === 0 && (
                <p className="text-center text-xs text-slate-400 py-4">暂无预算设置</p>
              )}
              {budgets.map((b) => {
                const ratio = Math.min((b.spentAmount / b.limitAmount) * 100, 100);
                const isWarning = ratio >= 80;
                return (
                  <div key={b.id} className="space-y-1.5">
                    <div className="flex justify-between items-center text-xs">
                      <span className="font-bold text-slate-700">{b.name}</span>
                      <span className={`font-extrabold font-mono ${isWarning ? 'text-rose-600' : 'text-slate-500'}`}>
                        {ratio.toFixed(1)}%
                      </span>
                    </div>
                    <div className="w-full bg-slate-150/60 rounded-full h-1.5 overflow-hidden">
                      <div
                        className={`h-full rounded-full transition-all duration-500 ${isWarning ? 'bg-rose-500' : 'bg-indigo-600'}`}
                        style={{ width: `${ratio}%` }}
                      ></div>
                    </div>
                    <div className="flex justify-between items-center text-[10px] text-slate-400 font-semibold">
                      <span>已支: ¥{b.spentAmount.toFixed(1)}</span>
                      <span>限额: ¥{b.limitAmount.toFixed(1)}</span>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
