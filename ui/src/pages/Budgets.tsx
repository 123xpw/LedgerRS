/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React, { useState } from 'react';
import { Ledger, Category, Budget } from '../types';
import {
  Plus,
  Trash2,
  ArrowLeft,
  Info,
  AlertTriangle,
  CheckCircle,
  FileText,
  Tag,
  Sliders,
  Shield
} from 'lucide-react';
import { LedgerIcon, getLedgerBgColor } from '../components/LedgerIcon';

interface BudgetsProps {
  ledger: Ledger;
  categories: Category[];
  budgets: Budget[];
  onCreateBudget: (data: { categoryName?: string; period: string; limitAmount: number }) => Promise<void>;
  onDeleteBudget: (id: string) => Promise<void>;
  setView: (view: 'transactions' | 'categories' | 'budgets' | 'ledgers' | 'dashboard') => void;
}

export function Budgets({ ledger, categories, budgets, onCreateBudget, onDeleteBudget, setView }: BudgetsProps) {
  const [showModal, setShowModal] = useState(false);
  const [categoryName, setCategoryName] = useState('');
  const [limitAmount, setLimitAmount] = useState('');
  const [period, setPeriod] = useState<'monthly' | 'weekly' | 'yearly'>('monthly');
  const [submitting, setSubmitting] = useState(false);
  const [modalError, setModalError] = useState('');

  const handleDelete = async (id: string) => {
    if (!confirm('确定要清除这个科目支出预算水位控制吗？清除后旧流水依旧存在，但不再进行水位累计预警。')) return;
    try {
      await onDeleteBudget(id);
    } catch (err) {
      alert(err instanceof Error ? err.message : '删除失败');
    }
  };

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!limitAmount || isNaN(parseFloat(limitAmount))) return;

    setSubmitting(true);
    setModalError('');

    try {
      await onCreateBudget({
        categoryName: categoryName || undefined,
        period,
        limitAmount: parseFloat(limitAmount),
      });
      setCategoryName('');
      setLimitAmount('');
      setShowModal(false);
    } catch (err) {
      setModalError(err instanceof Error ? err.message : '创建预算失败');
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 pb-1">
        <div>
          <div className="flex items-center gap-2 text-[11px] text-slate-400 font-bold uppercase tracking-wider mb-2">
            <button
              onClick={() => setView('ledgers')}
              className="hover:text-indigo-600 flex items-center gap-1 bg-none border-none cursor-pointer text-slate-400 font-bold transition-all"
            >
              <ArrowLeft className="w-3.5 h-3.5" />
              <span>账本目录</span>
            </button>
            <span>/</span>
            <span className="text-slate-600">本期预算水位安全阀</span>
          </div>
          <h1 className="text-2xl font-bold text-slate-900 tracking-tight flex items-center gap-2.5">
            <div className={`w-9 h-9 rounded-xl flex items-center justify-center border font-bold ${getLedgerBgColor(ledger.emoji, ledger.name)} shadow-xs`}>
              <LedgerIcon emoji={ledger.emoji} name={ledger.name} className="w-4.5 h-4.5" />
            </div>
            <span>{ledger.name} 专属预算设置</span>
          </h1>
        </div>

        <button
          onClick={() => { setCategoryName(''); setLimitAmount(''); setPeriod('monthly'); setModalError(''); setShowModal(true); }}
          className="bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-2.5 px-4 rounded-xl text-xs cursor-pointer transition-all flex items-center gap-1.5 shadow"
        >
          <Plus className="w-4 h-4" />
          <span>设定预算阀门</span>
        </button>
      </div>

      <div className="flex border-b border-slate-200 gap-1 mt-1">
        <button onClick={() => setView('transactions')} className="px-5 py-3 border-b-2 border-transparent text-slate-500 hover:text-slate-900 font-semibold text-xs flex items-center gap-2 cursor-pointer bg-transparent transition-all">
          <FileText className="w-4 h-4 text-slate-400" />
          <span>账目明细汇总</span>
        </button>
        <button onClick={() => setView('categories')} className="px-5 py-3 border-b-2 border-transparent text-slate-500 hover:text-slate-900 font-semibold text-xs flex items-center gap-2 cursor-pointer bg-transparent transition-all">
          <Tag className="w-4 h-4 text-slate-400" />
          <span>划分核算科目</span>
        </button>
        <button onClick={() => setView('budgets')} className="px-5 py-3 border-b-2 border-indigo-600 text-indigo-700 font-bold text-xs flex items-center gap-2 cursor-pointer bg-transparent">
          <Sliders className="w-4 h-4 text-indigo-600" />
          <span>本月预算限额</span>
          <span className="bg-indigo-100 text-indigo-700 px-2 py-0.5 rounded-full text-[9px] font-extrabold ml-1">{budgets.length}</span>
        </button>
      </div>

      <div className="bg-blue-50 border border-blue-150 rounded-xl p-4 text-blue-800 text-xs flex items-start gap-3">
        <Info className="w-4.5 h-4.5 text-blue-600 mt-0.5 flex-shrink-0" />
        <div className="space-y-1">
          <p className="font-extrabold text-blue-950">账内预算扣解机制</p>
          <p className="text-blue-700 leading-relaxed font-semibold">
            预算模块会在每笔「支出」账目计入账盘时，自核对应科目进行扣解。当水位占比达到 <strong className="font-extrabold text-rose-600">80% (警告阀限)</strong> 时，进度条将变为高频警告红。
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {budgets.map((b) => {
          const pct = Math.min((b.spentAmount / b.limitAmount) * 100, 100);
          const isWarning = pct >= 80;
          return (
            <div key={b.id} className="bg-white border border-slate-200 rounded-xl p-6 shadow-sm hover:shadow-md transition-all flex flex-col justify-between">
              <div>
                <div className="flex justify-between items-start mb-4">
                  <div>
                    <h3 className="font-extrabold text-sm text-slate-900 leading-tight mb-1">{b.name}</h3>
                    <span className="text-[9px] bg-slate-50 border border-slate-100 font-bold text-slate-500 rounded-md px-1.5 py-0.5 uppercase tracking-wide">
                      {b.period === 'monthly' ? '月度预算方案' : b.period === 'weekly' ? '周度控制' : '年度保障方案'}
                    </span>
                  </div>
                  <span className={`inline-flex items-center text-[10px] font-bold px-2.5 py-0.5 rounded-full border ${
                    isWarning ? 'bg-rose-50 text-rose-700 border-rose-100' : 'bg-emerald-50 text-emerald-700 border-emerald-100'
                  }`}>
                    {isWarning ? (
                      <div className="flex items-center gap-1"><AlertTriangle className="w-3 h-3 text-rose-500" /><span>水位告急</span></div>
                    ) : (
                      <div className="flex items-center gap-1"><CheckCircle className="w-3 h-3 text-emerald-500" /><span>水位安全</span></div>
                    )}
                  </span>
                </div>

                <div className="space-y-2 mt-5">
                  <div className="flex justify-between items-baseline text-xs">
                    <span className="text-slate-400 font-semibold">累计使用百分比</span>
                    <span className={`font-extrabold ${isWarning ? 'text-rose-600' : 'text-slate-800'}`}>{pct.toFixed(1)}%</span>
                  </div>
                  <div className="w-full bg-slate-100 rounded-full h-2 overflow-hidden border border-slate-50">
                    <div
                      className={`h-full rounded-full transition-all duration-300 ${isWarning ? 'bg-rose-500' : 'bg-indigo-600'}`}
                      style={{ width: `${pct}%` }}
                    ></div>
                  </div>
                  <div className="flex justify-between items-center text-[10.5px] text-slate-400 font-bold pt-1">
                    <span>已支出: ¥{b.spentAmount.toFixed(2)}</span>
                    <span>限额保障: ¥{b.limitAmount.toFixed(2)}</span>
                  </div>
                </div>
              </div>

              <div className="mt-6 pt-4 border-t border-slate-100 flex justify-end gap-2">
                <button
                  onClick={() => handleDelete(b.id)}
                  className="bg-rose-50 hover:bg-rose-100 border-none text-rose-600 hover:text-rose-700 font-bold flex items-center justify-center gap-1.5 w-full py-1.5 text-xs rounded-lg cursor-pointer transition-all"
                >
                  <Trash2 className="w-3.5 h-3.5" />
                  <span>撤销预警额</span>
                </button>
              </div>
            </div>
          );
        })}
        {budgets.length === 0 && (
          <div className="col-span-3 bg-white border border-slate-200 rounded-xl p-12 text-center text-slate-400">
            <span className="font-bold text-slate-700 block mb-1">未设定任何核算科目限额</span>
            <span className="text-xs">您可以点击右上角按钮在当前账本内增设科目水位！</span>
          </div>
        )}
      </div>

      {showModal && (
        <div className="fixed inset-0 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center z-50 animate-fade-in p-4">
          <div className="bg-white border border-slate-200 w-full max-w-[460px] rounded-2xl shadow-2xl overflow-hidden animate-slide-up">
            <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between bg-slate-50/60">
              <h2 className="text-sm font-bold text-slate-900 flex items-center gap-2">
                <Shield className="w-4 h-4 text-indigo-500" />
                <span>配置预算防超警戒线</span>
              </h2>
              <button onClick={() => setShowModal(false)} className="text-slate-400 hover:text-slate-600 text-lg font-bold cursor-pointer">×</button>
            </div>

            <form onSubmit={handleCreate} className="p-6 space-y-4">
              {modalError && (
                <div className="p-2.5 bg-rose-50 border border-rose-200 text-rose-800 text-xs rounded-lg font-medium">{modalError}</div>
              )}

              <div className="grid grid-cols-2 gap-4">
                <div className="form-group space-y-1.5">
                  <label className="text-xs font-bold text-slate-600 block">关联支出科目 (可不绑)</label>
                  <select
                    value={categoryName}
                    onChange={(e) => setCategoryName(e.target.value)}
                    className="w-full p-2.5 border border-slate-200 rounded-xl text-sm bg-slate-50/20 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all outline-none font-semibold"
                  >
                    <option value="">-- 全域全科目支出合计 --</option>
                    {categories.filter(c => c.type === 'expense').map(c => (
                      <option key={c.id} value={c.name}>{c.name}</option>
                    ))}
                  </select>
                </div>

                <div className="form-group space-y-1.5">
                  <label className="text-xs font-bold text-slate-600 block">审计时间周期</label>
                  <select
                    value={period}
                    onChange={(e) => setPeriod(e.target.value as 'monthly' | 'weekly' | 'yearly')}
                    className="w-full p-2.5 border border-slate-200 rounded-xl text-xs bg-slate-50/20 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all"
                  >
                    <option value="monthly">每月结算重置</option>
                    <option value="weekly">每周结算重置</option>
                    <option value="yearly">年度合并管理</option>
                  </select>
                </div>
              </div>

              <div className="form-group space-y-1.5">
                <label className="text-xs font-bold text-slate-600 block">水位限定保额上限 ({ledger.currency})</label>
                <input
                  type="number"
                  step="0.01"
                  required
                  placeholder="0.00"
                  value={limitAmount}
                  onChange={(e) => setLimitAmount(e.target.value)}
                  className="w-full p-2.5 border border-slate-200 rounded-xl text-sm bg-slate-50/20 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all outline-none font-semibold"
                />
              </div>

              <div className="flex justify-end gap-2.5 pt-4 border-t border-slate-100 mt-6">
                <button
                  type="button"
                  onClick={() => setShowModal(false)}
                  className="px-3.5 py-2.5 border border-slate-200 text-slate-600 font-semibold hover:bg-slate-50 rounded-xl text-xs cursor-pointer transition-all bg-white"
                >取消</button>
                <button
                  type="submit"
                  disabled={submitting}
                  className="px-4 py-2.5 bg-indigo-600 hover:bg-indigo-700 disabled:opacity-60 text-white font-bold rounded-xl text-xs cursor-pointer transition-all shadow-md shadow-indigo-100"
                >
                  {submitting ? '创建中...' : '确立安全阀'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}
