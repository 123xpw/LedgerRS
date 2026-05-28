/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React, { useState } from 'react';
import { Ledger, Category } from '../types';
import { CATEGORY_PRESETS } from '../mockData';
import {
  PlusCircle,
  Trash2,
  ArrowLeft,
  Sparkles,
  RefreshCw,
  FileText,
  Tag,
  Sliders,
  Check
} from 'lucide-react';
import { LedgerIcon, getLedgerBgColor } from '../components/LedgerIcon';

interface CategoriesProps {
  ledger: Ledger;
  categories: Category[];
  onCreateCategory: (data: { name: string; type: string; color: string }) => Promise<void>;
  onDeleteCategory: (id: string) => Promise<void>;
  onImportPreset: (cats: Array<{ name: string; type: string; color: string }>) => Promise<void>;
  setView: (view: 'transactions' | 'categories' | 'budgets' | 'ledgers' | 'dashboard') => void;
}

export function Categories({ ledger, categories, onCreateCategory, onDeleteCategory, onImportPreset, setView }: CategoriesProps) {
  const [showModal, setShowModal] = useState(false);
  const [catName, setCatName] = useState('');
  const [catType, setCatType] = useState<'expense' | 'income'>('expense');
  const [catColor, setCatColor] = useState('#1e2a5b');
  const [successMsg, setSuccessMsg] = useState('');
  const [errorMsg, setErrorMsg] = useState('');
  const [submitting, setSubmitting] = useState(false);

  const expenseCats = categories.filter(c => c.type === 'expense');
  const incomeCats = categories.filter(c => c.type === 'income');

  const handleDelete = async (id: string) => {
    if (!confirm('确定要删除这个分类吗？删除此账本类别，相关的旧交易记录依旧保留此类别名称，但无法继续在极速面板中进行选择折算。')) return;
    try {
      await onDeleteCategory(id);
      setSuccessMsg('分类已成功移除。');
      setTimeout(() => setSuccessMsg(''), 2500);
    } catch (err) {
      setErrorMsg(err instanceof Error ? err.message : '删除失败');
      setTimeout(() => setErrorMsg(''), 3000);
    }
  };

  const handleImportPreset = async (idx: number) => {
    const preset = CATEGORY_PRESETS[idx];
    const existing = new Set(categories.map(c => c.name.toLowerCase()));
    const toCreate = preset.categories.filter(c => !existing.has(c.name.toLowerCase()));

    if (toCreate.length === 0) {
      setSuccessMsg(`【${preset.name}】分类已全部存在！`);
      setTimeout(() => setSuccessMsg(''), 2500);
      return;
    }

    try {
      await onImportPreset(toCreate.map(c => ({ name: c.name, type: c.type, color: c.color })));
      setSuccessMsg(`【${preset.name}】预配收支科目已全量写入分类！`);
      setTimeout(() => setSuccessMsg(''), 3000);
    } catch (err) {
      setErrorMsg(err instanceof Error ? err.message : '导入失败');
      setTimeout(() => setErrorMsg(''), 3000);
    }
  };

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!catName.trim()) return;

    if (categories.some(c => c.name.toLowerCase() === catName.trim().toLowerCase() && c.type === catType)) {
      setErrorMsg('在同一种流向下，该科目分类已注册配置！');
      setTimeout(() => setErrorMsg(''), 3000);
      return;
    }

    setSubmitting(true);
    try {
      await onCreateCategory({ name: catName.trim(), type: catType, color: catColor });
      setCatName('');
      setShowModal(false);
      setSuccessMsg(`新增科目【${catName.trim()}】登记成功！`);
      setTimeout(() => setSuccessMsg(''), 2500);
    } catch (err) {
      setErrorMsg(err instanceof Error ? err.message : '创建失败');
      setTimeout(() => setErrorMsg(''), 3000);
    } finally {
      setSubmitting(false);
    }
  };

  const paletteColors = [
    '#ef4444', '#f97316', '#f59e0b', '#10b981', '#68a68c',
    '#3a4d7c', '#1e2a5b', '#5e6e8d', '#d946ef',
    '#ec4899', '#64748b'
  ];

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
            <span className="text-slate-600">独立收支科目划分</span>
          </div>
          <h1 className="text-2xl font-bold text-slate-900 tracking-tight flex items-center gap-2.5">
            <div className={`w-9 h-9 rounded-xl flex items-center justify-center border font-bold ${getLedgerBgColor(ledger.emoji, ledger.name)} shadow-xs`}>
              <LedgerIcon emoji={ledger.emoji} name={ledger.name} className="w-4.5 h-4.5" />
            </div>
            <span>{ledger.name} 专属科目设置</span>
          </h1>
        </div>

        <button
          onClick={() => { setCatName(''); setCatType('expense'); setCatColor('#1e2a5b'); setShowModal(true); }}
          className="bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-2.5 px-4 rounded-xl text-xs cursor-pointer transition-all flex items-center gap-1.5 shadow"
        >
          <PlusCircle className="w-4 h-4" />
          <span>新建核算科目</span>
        </button>
      </div>

      <div className="flex border-b border-slate-200 gap-1 mt-1">
        <button onClick={() => setView('transactions')} className="px-5 py-3 border-b-2 border-transparent text-slate-500 hover:text-slate-900 font-semibold text-xs flex items-center gap-2 cursor-pointer bg-transparent transition-all">
          <FileText className="w-4 h-4 text-slate-400" />
          <span>账目明细汇总</span>
        </button>
        <button onClick={() => setView('categories')} className="px-5 py-3 border-b-2 border-indigo-600 text-indigo-700 font-bold text-xs flex items-center gap-2 cursor-pointer bg-transparent">
          <Tag className="w-4 h-4 text-indigo-600" />
          <span>划分核算科目</span>
          <span className="bg-indigo-100 text-indigo-700 px-2 py-0.5 rounded-full text-[9px] font-extrabold ml-1">{categories.length}</span>
        </button>
        <button onClick={() => setView('budgets')} className="px-5 py-3 border-b-2 border-transparent text-slate-500 hover:text-slate-900 font-semibold text-xs flex items-center gap-2 cursor-pointer bg-transparent transition-all">
          <Sliders className="w-4 h-4 text-slate-400" />
          <span>本月预算限额</span>
        </button>
      </div>

      <div className="bg-slate-900 text-slate-100 border border-slate-800 rounded-xl p-5 shadow-lg relative overflow-hidden flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
        <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-500/25 rounded-full blur-2xl pointer-events-none"></div>
        <div className="space-y-1 relative z-10 max-w-xl">
          <div className="flex items-center gap-1.5">
            <Sparkles className="w-4 h-4 text-indigo-400" />
            <h3 className="font-bold text-xs text-indigo-300">行业专属核算维度推荐</h3>
          </div>
          <p className="text-[11px] text-slate-300 leading-normal">
            不需要琐碎手动加分类，一键选择适用场景：系统将自动导入一整套科学有度、颗粒度极佳的科目分类模型：
          </p>
        </div>
        <div className="flex flex-wrap gap-2 relative z-10 w-full md:w-auto">
          {CATEGORY_PRESETS.map((preset, idx) => (
            <button
              key={preset.name}
              onClick={() => handleImportPreset(idx)}
              className="flex items-center gap-1.5 px-3.5 py-2 bg-slate-800 border border-slate-700 hover:border-indigo-500 hover:bg-slate-750 rounded-xl text-[11px] font-bold cursor-pointer transition-all text-slate-200 shadow-2xs"
            >
              <RefreshCw className="w-3.5 h-3.5 text-indigo-400" />
              <span>导入【{preset.name}】科目包</span>
            </button>
          ))}
        </div>
      </div>

      {successMsg && (
        <div className="p-3 bg-emerald-50 border border-emerald-250/30 text-emerald-800 rounded-xl text-xs font-bold text-center">✓ {successMsg}</div>
      )}
      {errorMsg && (
        <div className="p-3 bg-rose-50 border border-rose-250/30 text-rose-800 rounded-xl text-xs font-bold text-center">⚠ {errorMsg}</div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {[
          { list: expenseCats, label: '支出科目类别', dot: 'bg-rose-500', badge: '负流向方', badgeCls: 'bg-rose-50 text-rose-600 border-rose-100/50' },
          { list: incomeCats, label: '收入科目类别', dot: 'bg-emerald-500', badge: '正流向方', badgeCls: 'bg-emerald-50 text-emerald-600 border-emerald-100/50' },
        ].map(({ list, label, dot, badge, badgeCls }) => (
          <div key={label} className="bg-white border border-slate-200 rounded-xl p-5 shadow-sm space-y-4">
            <div className="flex justify-between items-center pb-2.5 border-b border-slate-100">
              <h3 className="text-sm font-bold text-slate-800 flex items-center gap-1.5">
                <span className={`w-2.5 h-2.5 rounded-full ${dot}`}></span>
                <span>{label} ({list.length})</span>
              </h3>
              <span className={`text-[10px] font-extrabold rounded-full px-2 py-0.5 border ${badgeCls}`}>{badge}</span>
            </div>
            <div className="divide-y divide-slate-100/80 max-h-[400px] overflow-y-auto pr-1">
              {list.map((cat) => (
                <div key={cat.id} className="flex justify-between items-center py-3">
                  <div className="flex items-center gap-2.5">
                    <span className="w-3 h-3 rounded-full shadow-inner flex-shrink-0" style={{ backgroundColor: cat.color }}></span>
                    <span className="font-bold text-xs text-slate-800">{cat.name}</span>
                  </div>
                  <button
                    onClick={() => handleDelete(cat.id)}
                    className="p-1 text-slate-400 hover:text-rose-600 rounded bg-transparent border-none cursor-pointer transition-all"
                  >
                    <Trash2 className="w-4 h-4" />
                  </button>
                </div>
              ))}
              {list.length === 0 && (
                <div className="py-12 text-center text-slate-400 text-xs font-semibold">
                  暂无{label.includes('支出') ? '支出向' : '收入向'}科目，点击右上角按钮添加！
                </div>
              )}
            </div>
          </div>
        ))}
      </div>

      {showModal && (
        <div className="fixed inset-0 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center z-50 animate-fade-in p-4">
          <div className="bg-white border border-slate-200 w-full max-w-[460px] rounded-2xl shadow-2xl overflow-hidden animate-slide-up">
            <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between bg-slate-50/60">
              <h2 className="text-sm font-bold text-slate-900">✨ 新增核算细目分类</h2>
              <button onClick={() => setShowModal(false)} className="text-slate-400 hover:text-slate-600 text-lg font-bold cursor-pointer">×</button>
            </div>

            <form onSubmit={handleCreate} className="p-6 space-y-4">
              {errorMsg && (
                <div className="p-2.5 bg-rose-50 border border-rose-200 text-rose-800 text-xs rounded-lg font-medium">{errorMsg}</div>
              )}

              <div className="form-group space-y-1.5">
                <label className="text-xs font-bold text-slate-600 block">新科目的借贷类型</label>
                <div className="grid grid-cols-2 gap-2 bg-slate-100 p-1 rounded-xl border border-slate-200/30">
                  {(['expense', 'income'] as const).map((t) => (
                    <button
                      key={t}
                      type="button"
                      onClick={() => setCatType(t)}
                      className={`py-1.5 rounded-lg text-xs font-bold cursor-pointer transition-all ${
                        catType === t
                          ? `bg-white shadow-3xs ${t === 'expense' ? 'text-rose-600' : 'text-emerald-600'}`
                          : 'text-slate-500 hover:text-slate-850'
                      }`}
                    >
                      {t === 'expense' ? '支出类别' : '收入类别'}
                    </button>
                  ))}
                </div>
              </div>

              <div className="form-group space-y-1.5">
                <label className="text-xs font-bold text-slate-600 block">科目层级别称</label>
                <input
                  type="text"
                  required
                  placeholder="如: 服务器租金 / 健身年卡..."
                  value={catName}
                  onChange={(e) => setCatName(e.target.value)}
                  className="w-full p-2.5 border border-slate-200 rounded-xl text-sm bg-slate-50/20 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all outline-none font-semibold"
                />
              </div>

              <div className="form-group space-y-1.5">
                <label className="text-xs font-bold text-slate-600 block">科目归底索引标注色</label>
                <div className="grid grid-cols-6 gap-2.5 pt-1">
                  {paletteColors.map((color) => (
                    <button
                      type="button"
                      key={color}
                      onClick={() => setCatColor(color)}
                      className="w-9 h-9 rounded-full relative shadow-sm border border-slate-100 flex items-center justify-center cursor-pointer transition-transform hover:scale-110 flex-shrink-0"
                      style={{ backgroundColor: color }}
                    >
                      {catColor === color && <Check className="w-4 h-4 text-white drop-shadow-sm font-extrabold" />}
                    </button>
                  ))}
                </div>
              </div>

              <div className="flex justify-end gap-2.5 pt-4 border-t border-slate-100 mt-6">
                <button
                  type="button"
                  onClick={() => setShowModal(false)}
                  className="px-3.5 py-2.5 border border-slate-200 text-slate-600 font-semibold hover:bg-slate-50 rounded-lg text-xs cursor-pointer transition-all bg-white"
                >取消</button>
                <button
                  type="submit"
                  disabled={submitting}
                  className="px-4 py-2.5 bg-indigo-600 hover:bg-indigo-700 disabled:opacity-60 text-white font-bold rounded-xl text-xs cursor-pointer transition-all shadow-md shadow-indigo-100"
                >
                  {submitting ? '创建中...' : '确认设立'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}
