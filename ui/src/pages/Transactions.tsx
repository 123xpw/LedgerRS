/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React, { useState } from 'react';
import { Ledger, Transaction, Category } from '../types';
import {
  Search,
  ArrowLeft,
  Calendar,
  Trash2,
  Download,
  Plus,
  FileText,
  Tag,
  Sliders,
  Shield
} from 'lucide-react';
import { LedgerIcon, getLedgerBgColor } from '../components/LedgerIcon';

interface TransactionsProps {
  ledger: Ledger;
  transactions: Transaction[];
  categories: Category[];
  addTransaction: (tx: Omit<Transaction, 'id'>) => Promise<void>;
  deleteTransaction: (id: string) => Promise<void>;
  setView: (view: 'transactions' | 'categories' | 'budgets' | 'ledgers' | 'dashboard') => void;
}

export function Transactions({
  ledger,
  transactions,
  categories,
  addTransaction,
  deleteTransaction,
  setView
}: TransactionsProps) {
  const [showModal, setShowModal] = useState(false);
  const [filterType, setFilterType] = useState<'all' | 'expense' | 'income'>('all');
  const [searchQuery, setSearchQuery] = useState('');
  const [currentPage, setCurrentPage] = useState(1);
  const itemsPerPage = 5;

  const [amount, setAmount] = useState('');
  const [type, setType] = useState<'expense' | 'income'>('expense');
  const [category, setCategory] = useState('');
  const [note, setNote] = useState('');
  const [submitting, setSubmitting] = useState(false);
  const [modalError, setModalError] = useState('');

  const filteredTxs = transactions.filter((t) => {
    const matchesType = filterType === 'all' || t.type === filterType;
    const matchesSearch =
      t.category.toLowerCase().includes(searchQuery.toLowerCase()) ||
      t.note.toLowerCase().includes(searchQuery.toLowerCase());
    return matchesType && matchesSearch;
  });

  const pageCount = Math.ceil(filteredTxs.length / itemsPerPage) || 1;
  const startIndex = (currentPage - 1) * itemsPerPage;
  const currentItems = filteredTxs.slice(startIndex, startIndex + itemsPerPage);

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!amount || isNaN(parseFloat(amount))) return;

    const chosenCat = category || (categories.filter(c => c.type === type)[0]?.name || '其他');
    setSubmitting(true);
    setModalError('');

    try {
      await addTransaction({
        date: new Date().toISOString().split('T')[0],
        type,
        amount: parseFloat(parseFloat(amount).toFixed(2)),
        currency: ledger.currency,
        category: chosenCat,
        note: note || '备记'
      });
      setAmount('');
      setNote('');
      setShowModal(false);
    } catch (err) {
      setModalError(err instanceof Error ? err.message : '记账失败');
    } finally {
      setSubmitting(false);
    }
  };

  const handleDelete = async (id: string) => {
    if (!confirm('确定要撤回删除该笔交易流水吗？相关的账本余额及预算扣缴将重新折算归档。')) return;
    try {
      await deleteTransaction(id);
    } catch (err) {
      alert(err instanceof Error ? err.message : '删除失败');
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
            <span className="text-slate-600">独立账目明细汇总</span>
          </div>
          <h1 className="text-2xl font-bold text-slate-900 tracking-tight flex items-center gap-2.5">
            <div className={`w-9 h-9 rounded-xl flex items-center justify-center border font-bold ${getLedgerBgColor(ledger.emoji, ledger.name)} shadow-xs`}>
              <LedgerIcon emoji={ledger.emoji} name={ledger.name} className="w-4.5 h-4.5" />
            </div>
            <span>{ledger.name}</span>
          </h1>
        </div>

        <div className="flex items-center gap-2.5">
          <button
            onClick={() => alert('已成功生成单账簿加密 CSV 数据包，将在浏览器中开始下载。')}
            className="px-3.5 py-2.5 border border-slate-200 text-slate-700 font-bold hover:border-indigo-450 hover:text-indigo-600 rounded-xl text-xs cursor-pointer bg-white transition-all flex items-center gap-1.5 shadow-3xs"
          >
            <Download className="w-4 h-4 text-slate-400" />
            <span>导出单账簿数据</span>
          </button>
          <button
            onClick={() => {
              setType('expense');
              setAmount('');
              setCategory('');
              setNote('');
              setModalError('');
              setShowModal(true);
            }}
            className="bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-2.5 px-4 rounded-xl text-xs cursor-pointer transition-all flex items-center gap-1.5 shadow"
          >
            <Plus className="w-4 h-4" />
            <span>新增账目登记</span>
          </button>
        </div>
      </div>

      <div className="flex border-b border-slate-200 gap-1 mt-1">
        <button
          onClick={() => setView('transactions')}
          className="px-5 py-3 border-b-2 border-indigo-600 text-indigo-700 font-bold text-xs flex items-center gap-2 cursor-pointer bg-transparent"
        >
          <FileText className="w-4 h-4 text-indigo-600" />
          <span>账目明细汇总</span>
          <span className="bg-indigo-100 text-indigo-700 px-2 py-0.5 rounded-full text-[9px] font-extrabold ml-1">
            {transactions.length}
          </span>
        </button>
        <button
          onClick={() => setView('categories')}
          className="px-5 py-3 border-b-2 border-transparent text-slate-500 hover:text-slate-900 font-semibold text-xs flex items-center gap-2 cursor-pointer bg-transparent transition-all"
        >
          <Tag className="w-4 h-4 text-slate-400" />
          <span>划分核算科目</span>
        </button>
        <button
          onClick={() => setView('budgets')}
          className="px-5 py-3 border-b-2 border-transparent text-slate-500 hover:text-slate-900 font-semibold text-xs flex items-center gap-2 cursor-pointer bg-transparent transition-all"
        >
          <Sliders className="w-4 h-4 text-slate-400" />
          <span>本月预算限额</span>
        </button>
      </div>

      <div className="bg-white border border-slate-200 rounded-xl p-4 shadow-3xs flex flex-col md:flex-row gap-4 items-center justify-between">
        <div className="flex items-center gap-2.5 w-full md:w-auto">
          <div className="flex bg-slate-100 p-1 rounded-xl border border-slate-200/40">
            {(['all', 'expense', 'income'] as const).map((f) => (
              <button
                key={f}
                onClick={() => { setFilterType(f); setCurrentPage(1); }}
                className={`px-3.5 py-1.5 rounded-lg text-xs font-semibold cursor-pointer transition-all ${
                  filterType === f
                    ? `bg-white shadow-3xs ${f === 'expense' ? 'text-rose-600' : f === 'income' ? 'text-emerald-600' : 'text-slate-900'}`
                    : `text-slate-500 ${f === 'expense' ? 'hover:text-rose-600/95' : f === 'income' ? 'hover:text-emerald-500' : 'hover:text-slate-800'}`
                }`}
              >
                {f === 'all' ? '全部交易' : f === 'expense' ? '支出款' : '收入款'}
              </button>
            ))}
          </div>
        </div>

        <div className="relative w-full md:w-72">
          <Search className="absolute left-3.5 top-2.5 w-4 h-4 text-slate-400" />
          <input
            type="text"
            placeholder="搜索科目分类或明细记叙..."
            value={searchQuery}
            onChange={(e) => { setSearchQuery(e.target.value); setCurrentPage(1); }}
            className="w-full pl-9 pr-4 py-2 text-xs border border-slate-200 rounded-xl outline-none focus:border-indigo-600 focus:ring-4 focus:ring-indigo-100/50 transition-all font-semibold"
          />
        </div>
      </div>

      <div className="bg-white border border-slate-200 rounded-xl shadow-sm overflow-hidden">
        <div className="overflow-x-auto">
          <table className="w-full border-collapse text-left text-xs">
            <thead>
              <tr className="bg-slate-50/70 border-b border-slate-200 text-slate-500 uppercase font-extrabold tracking-wider">
                <th className="px-6 py-3.5">交易发生时间</th>
                <th className="px-6 py-3.5">资金往来流向</th>
                <th className="px-6 py-3.5">收支金额</th>
                <th className="px-6 py-3.5">科目归核分类</th>
                <th className="px-6 py-3.5">摘要/备忘细则</th>
                <th className="px-6 py-3.5 text-right">账目操控</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-slate-100 font-semibold text-slate-700">
              {currentItems.map((t) => (
                <tr key={t.id} className="hover:bg-slate-50/20 transition-colors">
                  <td className="px-6 py-4 text-slate-500 whitespace-nowrap">
                    <div className="flex items-center gap-1.5">
                      <Calendar className="w-3.5 h-3.5 text-slate-400" />
                      <span>{t.date}</span>
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    {t.type === 'expense' ? (
                      <span className="inline-flex items-center text-[10px] font-bold bg-rose-50 border border-rose-100 text-rose-700 px-2 py-0.5 rounded-full">支出款项</span>
                    ) : t.type === 'income' ? (
                      <span className="inline-flex items-center text-[10px] font-bold bg-emerald-50 border border-emerald-100 text-emerald-700 px-2 py-0.5 rounded-full">收入款项</span>
                    ) : (
                      <span className="inline-flex items-center text-[10px] font-bold bg-indigo-50 border border-indigo-100 text-indigo-700 px-2 py-0.5 rounded-full">转账</span>
                    )}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span className={`text-sm font-extrabold ${t.type === 'expense' ? 'text-rose-600' : 'text-emerald-600'}`}>
                      {t.type === 'expense' ? '-' : '+'} {t.amount.toFixed(2)}
                    </span>
                    <span className="text-[10px] text-slate-400 font-bold ml-1">{t.currency}</span>
                  </td>
                  <td className="px-6 py-4 text-slate-800 font-bold whitespace-nowrap">
                    <span className="px-2 py-1 bg-slate-100 border border-slate-250/30 rounded-lg text-[10px]">
                      {t.category}
                    </span>
                  </td>
                  <td className="px-6 py-4 text-slate-500 truncate max-w-xs">{t.note}</td>
                  <td className="px-6 py-4 text-right whitespace-nowrap">
                    <button
                      onClick={() => handleDelete(t.id)}
                      className="text-rose-500 hover:text-rose-700 font-bold bg-transparent border-none cursor-pointer flex items-center gap-1 ml-auto"
                    >
                      <Trash2 className="w-3.5 h-3.5" />
                      <span>删除</span>
                    </button>
                  </td>
                </tr>
              ))}
              {filteredTxs.length === 0 && (
                <tr>
                  <td colSpan={6} className="px-6 py-12 text-center text-slate-400 leading-relaxed font-normal">
                    <p className="font-semibold text-slate-700 mb-0.5">该账簿下无任何匹配款项交易</p>
                    <p className="text-xs">您可以点击右上角的"新增账目登记"录入第一笔消费！</p>
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>

        <div className="px-6 py-4 bg-slate-50/50 border-t border-slate-200/70 flex justify-between items-center">
          <span className="text-xs text-slate-500 font-medium">
            本期核算下共包含 <strong className="font-extrabold text-slate-800">{filteredTxs.length}</strong> 笔匹配记录
          </span>
          <div className="flex bg-white border border-slate-200 rounded-xl p-1 items-center gap-1.5 shadow-3xs">
            <button
              onClick={() => setCurrentPage(prev => Math.max(prev - 1, 1))}
              disabled={currentPage === 1}
              className="px-2.5 py-1 hover:bg-slate-50 text-slate-600 disabled:opacity-40 text-xs font-semibold cursor-pointer rounded-lg transition-all"
            >上一页</button>
            <span className="text-xs text-slate-500 px-1 font-bold">{currentPage} / {pageCount}</span>
            <button
              onClick={() => setCurrentPage(prev => Math.min(prev + 1, pageCount))}
              disabled={currentPage === pageCount}
              className="px-2.5 py-1 hover:bg-slate-50 text-slate-600 disabled:opacity-40 text-xs font-semibold cursor-pointer rounded-lg transition-all"
            >下一页</button>
          </div>
        </div>
      </div>

      {showModal && (
        <div className="fixed inset-0 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center z-50 animate-fade-in p-4">
          <div className="bg-white border border-slate-200 w-full max-w-[460px] rounded-2xl shadow-2xl overflow-hidden animate-slide-up">
            <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between bg-slate-50/60">
              <h2 className="text-sm font-bold text-slate-900 flex items-center gap-2">
                <Shield className="w-4 h-4 text-indigo-500" />
                <span>安全登记录入账单</span>
              </h2>
              <button onClick={() => setShowModal(false)} className="text-slate-400 hover:text-slate-600 text-lg font-bold cursor-pointer">×</button>
            </div>

            <form onSubmit={handleCreate} className="p-6 space-y-4">
              {modalError && (
                <div className="p-2.5 bg-rose-50 border border-rose-200 text-rose-800 text-xs rounded-lg font-medium">{modalError}</div>
              )}

              <div className="form-group space-y-1.5">
                <label className="text-xs font-bold text-slate-600 block">资金交易流向</label>
                <div className="grid grid-cols-2 gap-2 bg-slate-100 p-1 rounded-xl border border-slate-200/30">
                  {(['expense', 'income'] as const).map((t) => (
                    <button
                      key={t}
                      type="button"
                      onClick={() => { setType(t); setCategory(''); }}
                      className={`py-1.5 rounded-lg text-xs font-bold cursor-pointer transition-all ${
                        type === t
                          ? `bg-white shadow-3xs ${t === 'expense' ? 'text-rose-600' : 'text-emerald-600'}`
                          : 'text-slate-500 hover:text-slate-800'
                      }`}
                    >
                      {t === 'expense' ? '支出款项' : '收入款项'}
                    </button>
                  ))}
                </div>
              </div>

              <div className="form-group space-y-1.5">
                <label className="text-xs font-bold text-slate-600 block">结付分类项目</label>
                <select
                  value={category}
                  onChange={(e) => setCategory(e.target.value)}
                  className="w-full p-2.5 border border-slate-200 rounded-xl text-sm bg-slate-50/20 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all outline-none font-semibold"
                >
                  {categories.filter(c => c.type === type).map(c => (
                    <option key={c.id} value={c.name}>{c.name}</option>
                  ))}
                  {categories.filter(c => c.type === type).length === 0 && (
                    <option value={type === 'expense' ? '日常餐饮' : '劳动薪资'}>
                      {type === 'expense' ? '日常餐饮' : '劳动薪资'} (默认)
                    </option>
                  )}
                </select>
              </div>

              <div className="form-group space-y-1.5">
                <label className="text-xs font-bold text-slate-600 block">结算借贷金额 ({ledger.currency})</label>
                <input
                  type="number"
                  step="0.01"
                  required
                  placeholder="0.00"
                  value={amount}
                  onChange={(e) => setAmount(e.target.value)}
                  className="w-full p-2.5 border border-slate-200 rounded-xl text-sm bg-slate-50/20 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all outline-none font-semibold"
                />
              </div>

              <div className="form-group space-y-1.5">
                <label className="text-xs font-bold text-slate-600 block">摘要/记事补充</label>
                <input
                  type="text"
                  placeholder="吃什么、买什么或者是由于何种理由入账..."
                  value={note}
                  onChange={(e) => setNote(e.target.value)}
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
                  className="px-4 py-2.5 bg-indigo-600 hover:bg-indigo-700 disabled:opacity-60 text-white font-bold rounded-xl text-xs cursor-pointer transition-all shadow shadow-indigo-100"
                >
                  {submitting ? '记账中...' : '记账确立'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}
