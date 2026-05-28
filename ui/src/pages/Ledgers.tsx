/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React, { useState } from 'react';
import { Ledger } from '../types';
import { Plus, Edit2, Trash2, FileText, Tag, Sliders } from 'lucide-react';
import { LedgerIcon, getLedgerBgColor } from '../components/LedgerIcon';

interface LedgersProps {
  ledgers: Ledger[];
  onCreateLedger: (data: { name: string; emoji: string; currency: string; balance: number }) => Promise<void>;
  onUpdateLedger: (id: string, data: { name: string; emoji: string; balance: number }) => Promise<void>;
  onDeleteLedger: (id: string) => Promise<void>;
  setSelectedLedgerId: (id: string) => void;
  setView: (view: 'transactions' | 'categories' | 'budgets' | 'ledgers' | 'dashboard') => void;
}

export function Ledgers({ ledgers, onCreateLedger, onUpdateLedger, onDeleteLedger, setSelectedLedgerId, setView }: LedgersProps) {
  const [showModal, setShowModal] = useState(false);
  const [modalType, setModalType] = useState<'create' | 'edit'>('create');
  const [activeLedgerId, setActiveLedgerId] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);
  const [modalError, setModalError] = useState('');

  const [name, setName] = useState('');
  const [emoji, setEmoji] = useState('💳');
  const [currency, setCurrency] = useState('CNY');
  const [balance, setBalance] = useState('');

  const handleOpenCreate = () => {
    setModalType('create');
    setName('');
    setEmoji('💳');
    setCurrency('CNY');
    setBalance('');
    setModalError('');
    setShowModal(true);
  };

  const handleOpenEdit = (l: Ledger) => {
    setModalType('edit');
    setActiveLedgerId(l.id);
    setName(l.name);
    setEmoji(l.emoji);
    setCurrency(l.currency);
    setBalance(l.balance.toString());
    setModalError('');
    setShowModal(true);
  };

  const handleDelete = async (id: string) => {
    if (!confirm('确定要彻底删除该账本吗？此账本下的所有交易明细、分类账、预算管理设置将永久不复存在。')) return;
    try {
      await onDeleteLedger(id);
    } catch (err) {
      alert(err instanceof Error ? err.message : '删除失败');
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;

    const parsedBal = parseFloat(balance) || 0;
    setSubmitting(true);
    setModalError('');

    try {
      if (modalType === 'create') {
        await onCreateLedger({ name, emoji, currency, balance: parsedBal });
      } else if (modalType === 'edit' && activeLedgerId) {
        await onUpdateLedger(activeLedgerId, { name, emoji, balance: parsedBal });
      }
      setShowModal(false);
    } catch (err) {
      setModalError(err instanceof Error ? err.message : '操作失败');
    } finally {
      setSubmitting(false);
    }
  };

  const handleSubRouteRedirect = (id: string, subview: 'transactions' | 'categories' | 'budgets') => {
    setSelectedLedgerId(id);
    setView(subview);
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center pb-3 mb-2 border-b border-slate-200/60">
        <div>
          <h1 className="text-xl font-extrabold text-[#0F172A] tracking-tight">我的多币种账簿</h1>
          <p className="text-xs text-slate-500 mt-1">
            独立追踪您的多维度开支、境外代购外币流或日常备用现金，每套账簿拥有独立的类目和防超支警戒。
          </p>
        </div>
        <button
          onClick={handleOpenCreate}
          className="bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-2 px-4 rounded-xl text-xs cursor-pointer transition-all flex items-center gap-1 shadow-sm shadow-indigo-100"
        >
          <Plus className="w-4 h-4" />
          <span>创建新账簿</span>
        </button>
      </div>

      {ledgers.length === 0 && (
        <div className="bg-white border border-slate-200 rounded-2xl p-12 text-center text-slate-400">
          <p className="font-semibold text-slate-700 mb-1">还没有账本</p>
          <p className="text-xs">点击右上角「创建新账簿」开始记账之旅！</p>
        </div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {ledgers.map((l) => (
          <div
            key={l.id}
            className="bg-white border border-slate-200 rounded-2xl p-6 hover:border-indigo-400 hover:shadow-xs transition-all flex flex-col justify-between group overflow-hidden relative"
          >
            <div className="relative z-10 flex flex-col justify-between h-full w-full">
              <div>
                <div className="flex justify-between items-start mb-4">
                  <div className={`w-12 h-12 rounded-2xl flex items-center justify-center border font-bold ${getLedgerBgColor(l.emoji, l.name)} shadow-3xs`}>
                    <LedgerIcon emoji={l.emoji} name={l.name} className="w-5 h-5" />
                  </div>
                  <span className="text-xs font-bold text-slate-500 bg-slate-50 px-2.5 py-0.5 rounded-full border border-slate-200/60">
                    {l.currency}
                  </span>
                </div>

                <h3 className="font-bold text-sm text-slate-900 mb-1">{l.name}</h3>
                <p className="text-[11px] text-slate-400">核算模式: 独立币种结算本</p>

                <div className="mt-4 pb-4 border-b border-slate-100">
                  <span className="text-[10px] text-slate-400 block font-bold uppercase tracking-wider mb-0.5">账面结余本金</span>
                  <span className="text-xl font-bold text-slate-900 tracking-tight font-mono">
                    {l.currency === 'USD' ? '$' : '¥'} {l.balance.toLocaleString('zh-CN', { minimumFractionDigits: 2 })}
                  </span>
                </div>
              </div>

              <div className="mt-4 py-3 bg-slate-50/70 border border-slate-100/50 rounded-xl p-3 space-y-2">
                <span className="text-[9px] font-bold text-slate-400 uppercase tracking-wider block">快捷配置</span>
                <div className="grid grid-cols-3 gap-1.5 text-center">
                  <button
                    onClick={() => handleSubRouteRedirect(l.id, 'transactions')}
                    className="px-1 py-1.5 bg-white border border-slate-200/90 hover:border-indigo-500 text-slate-700 hover:text-indigo-600 rounded-lg text-[10px] font-bold cursor-pointer transition-all shadow-3xs flex flex-col items-center justify-center gap-1"
                  >
                    <FileText className="w-3.5 h-3.5 text-slate-400" />
                    <span>账目流水</span>
                  </button>
                  <button
                    onClick={() => handleSubRouteRedirect(l.id, 'categories')}
                    className="px-1 py-1.5 bg-white border border-slate-200/90 hover:border-indigo-500 text-slate-700 hover:text-indigo-600 rounded-lg text-[10px] font-bold cursor-pointer transition-all shadow-3xs flex flex-col items-center justify-center gap-1"
                  >
                    <Tag className="w-3.5 h-3.5 text-slate-400" />
                    <span>分类标签</span>
                  </button>
                  <button
                    onClick={() => handleSubRouteRedirect(l.id, 'budgets')}
                    className="px-1 py-1.5 bg-white border border-slate-200/90 hover:border-indigo-500 text-slate-700 hover:text-indigo-600 rounded-lg text-[10px] font-bold cursor-pointer transition-all shadow-3xs flex flex-col items-center justify-center gap-1"
                  >
                    <Sliders className="w-3.5 h-3.5 text-slate-400" />
                    <span>防超预算</span>
                  </button>
                </div>
              </div>

              <div className="mt-5 pt-3 border-t border-slate-100 flex justify-end gap-2">
                <button
                  onClick={() => handleOpenEdit(l)}
                  className="btn-ghost flex items-center gap-1 py-1 px-3 text-[11px] bg-slate-100 hover:bg-slate-200/80 rounded-lg font-semibold text-slate-700 cursor-pointer"
                >
                  <Edit2 className="w-3 h-3 text-slate-500" />
                  <span>修改</span>
                </button>
                <button
                  onClick={() => handleDelete(l.id)}
                  className="btn bg-rose-50 hover:bg-rose-100 border-none text-rose-600 hover:text-rose-700 font-bold flex items-center gap-1 py-1 px-3 text-[11px] rounded-lg cursor-pointer transition-all"
                >
                  <Trash2 className="w-3 h-3" />
                  <span>删除</span>
                </button>
              </div>
            </div>
          </div>
        ))}
      </div>

      {showModal && (
        <div className="fixed inset-0 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center z-50 animate-fade-in p-4">
          <div className="bg-white border border-slate-200 w-full max-w-[460px] rounded-2xl shadow-2xl overflow-hidden animate-slide-up">
            <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between bg-slate-50/60">
              <h2 className="text-sm font-bold text-slate-900">
                {modalType === 'create' ? '✨ 创建独立功能账本' : '📝 修改账本基本配置'}
              </h2>
              <button onClick={() => setShowModal(false)} className="text-slate-400 hover:text-slate-600 text-lg font-bold cursor-pointer">×</button>
            </div>

            <form onSubmit={handleSubmit} className="p-6 space-y-4">
              {modalError && (
                <div className="p-2.5 bg-rose-50 border border-rose-200 text-rose-800 text-xs rounded-lg font-medium">{modalError}</div>
              )}

              <div className="form-group space-y-1.5">
                <label className="text-xs font-bold text-slate-600 block">账本别称</label>
                <input
                  type="text"
                  required
                  placeholder="如: 个人咖啡手账 / 家庭共用卡"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  className="w-full p-2 border border-slate-200 rounded-lg text-sm bg-slate-50/30 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all outline-none"
                />
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div className="form-group space-y-1.5">
                  <label className="text-xs font-bold text-slate-600 block">类型图标</label>
                  <select
                    value={emoji}
                    onChange={(e) => setEmoji(e.target.value)}
                    className="w-full p-2 border border-slate-200 rounded-lg text-sm bg-slate-50/30 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all outline-none"
                  >
                    <option value="💳">💳 银行卡/信用卡</option>
                    <option value="✈️">✈️ 出国考察/海淘卡</option>
                    <option value="💑">💑 情侣共济/小金库</option>
                    <option value="💰">💰 现金钱包/猪猪储蓄</option>
                  </select>
                </div>

                <div className="form-group space-y-1.5">
                  <label className="text-xs font-bold text-slate-600 block">核算币种</label>
                  <select
                    value={currency}
                    onChange={(e) => setCurrency(e.target.value)}
                    disabled={modalType === 'edit'}
                    className="w-full p-2 border border-slate-200 rounded-lg text-sm bg-slate-50/30 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all disabled:opacity-50"
                  >
                    <option value="CNY">CNY - 人民币元</option>
                    <option value="USD">USD - 美元</option>
                    <option value="EUR">EUR - 欧元</option>
                    <option value="JPY">JPY - 日元</option>
                    <option value="GBP">GBP - 英镑</option>
                  </select>
                  {modalType === 'edit' && <p className="text-[10px] text-slate-400">币种创建后不可修改</p>}
                </div>
              </div>

              <div className="form-group space-y-1.5">
                <label className="text-xs font-bold text-slate-600 block">账本初始余额</label>
                <input
                  type="number"
                  step="0.01"
                  placeholder="0.00"
                  value={balance}
                  onChange={(e) => setBalance(e.target.value)}
                  className="w-full p-2 border border-slate-200 rounded-lg text-sm bg-slate-50/30 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all outline-none"
                />
              </div>

              <div className="flex justify-end gap-2.5 pt-4 border-t border-slate-100 mt-6">
                <button
                  type="button"
                  onClick={() => setShowModal(false)}
                  className="px-3.5 py-2 border border-slate-200 text-slate-600 font-semibold hover:bg-slate-50 rounded-lg text-xs cursor-pointer transition-all bg-white"
                >
                  取消
                </button>
                <button
                  type="submit"
                  disabled={submitting}
                  className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 disabled:opacity-60 text-white font-bold rounded-lg text-xs cursor-pointer transition-all shadow-md shadow-indigo-100"
                >
                  {submitting ? '保存中...' : '保存信息'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}
