/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React, { useState, useEffect, useCallback } from 'react';
import { Sidebar } from './components/Sidebar';
import { Auth } from './pages/Auth';
import { Dashboard } from './pages/Dashboard';
import { Ledgers } from './pages/Ledgers';
import { Transactions } from './pages/Transactions';
import { Categories } from './pages/Categories';
import { Budgets } from './pages/Budgets';
import { Reports } from './pages/Reports';
import { Settings } from './pages/Settings';
import { User, Ledger, Transaction, Category, Budget, ViewType } from './types';
import {
  getStoredAuth,
  clearStoredAuth,
  apiMe,
  apiGetLedgers,
  apiGetLedger,
  apiCreateLedger,
  apiUpdateLedger,
  apiDeleteLedger,
  apiGetTransactions,
  apiCreateTransaction,
  apiDeleteTransaction,
  apiGetCategories,
  apiCreateCategory,
  apiDeleteCategory,
  apiGetBudgets,
  apiCreateBudget,
  apiDeleteBudget,
  ApiLedger,
  ApiTransaction,
  ApiCategory,
  ApiBudget,
} from './api';

// ─── Color palette for categories ────────────────────────────────────────────

const COLORS = [
  '#ef4444', '#f97316', '#f59e0b', '#10b981', '#68a68c',
  '#3a4d7c', '#1e2a5b', '#5e6e8d', '#d946ef',
  '#ec4899', '#64748b', '#6366f1', '#0ea5e9'
];

function assignColor(id: string): string {
  let hash = 0;
  for (let i = 0; i < id.length; i++) {
    hash = ((hash << 5) - hash) + id.charCodeAt(i);
    hash |= 0;
  }
  return COLORS[Math.abs(hash) % COLORS.length];
}

// ─── API response mappers ─────────────────────────────────────────────────────

function mapLedger(r: ApiLedger): Ledger {
  return {
    id: r.id,
    name: r.name,
    emoji: r.icon || '💳',
    currency: r.base_currency,
    balance: parseFloat(r.balance) || 0,
    thisMonthIncome: parseFloat(r.this_month_income) || 0,
    thisMonthExpense: parseFloat(r.this_month_expense) || 0,
  };
}

function mapTransaction(r: ApiTransaction): Transaction {
  return {
    id: r.id,
    date: r.occurred_at.split('T')[0],
    type: r.type as 'expense' | 'income' | 'transfer',
    amount: parseFloat(r.amount) || 0,
    currency: r.currency,
    category: r.category_name || '其他',
    note: r.note || '',
  };
}

function mapCategory(r: ApiCategory): Category {
  return {
    id: r.id,
    name: r.name,
    type: r.type as 'expense' | 'income',
    color: assignColor(r.id),
  };
}

function mapBudget(r: ApiBudget, categories: Category[]): Budget {
  const cat = categories.find(c => c.id === r.category_id);
  const periodLabel = r.period === 'month' ? '月度' : r.period === 'week' ? '周度' : '年度';
  const name = cat ? `${periodLabel}${cat.name}限额` : `${periodLabel}总预算`;
  const uiPeriod: 'monthly' | 'weekly' | 'yearly' =
    r.period === 'month' ? 'monthly' : r.period === 'week' ? 'weekly' : 'yearly';

  return {
    id: r.id,
    name,
    category: cat?.name,
    period: uiPeriod,
    limitAmount: parseFloat(r.amount) || 0,
    spentAmount: parseFloat(r.spent) || 0,
  };
}

// ─── Period UI → API mapping ──────────────────────────────────────────────────

function uiPeriodToApi(p: string): string {
  if (p === 'monthly') return 'month';
  if (p === 'weekly') return 'week';
  if (p === 'yearly') return 'year';
  return 'month';
}

// ─── Main App ─────────────────────────────────────────────────────────────────

export default function App() {
  const [user, setUser] = useState<User | null>(null);
  const [ledgers, setLedgers] = useState<Ledger[]>([]);
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [categories, setCategories] = useState<Category[]>([]);
  const [budgets, setBudgets] = useState<Budget[]>([]);
  const [currentView, setView] = useState<ViewType>('dashboard');
  const [selectedLedgerId, setSelectedLedgerId] = useState<string>('');
  const [loading, setLoading] = useState(true);
  const [triggerExpNotice, setTriggerExpNotice] = useState(false);

  // ── Load ledgers ──────────────────────────────────────────────────────────

  const loadLedgers = useCallback(async () => {
    const apiLedgers = await apiGetLedgers();
    const mapped = apiLedgers.map(mapLedger);
    setLedgers(mapped);
    return mapped;
  }, []);

  // ── Load per-ledger data ──────────────────────────────────────────────────

  const loadLedgerData = useCallback(async (ledgerId: string) => {
    const [txResp, catResp, budResp] = await Promise.all([
      apiGetTransactions(ledgerId, 1, 100),
      apiGetCategories(ledgerId),
      apiGetBudgets(ledgerId),
    ]);
    const mappedCats = catResp.map(mapCategory);
    setTransactions(txResp.items.map(mapTransaction));
    setCategories(mappedCats);
    setBudgets(budResp.map(b => mapBudget(b, mappedCats)));
  }, []);

  // ── Bootstrap: check auth on mount ───────────────────────────────────────

  useEffect(() => {
    const stored = getStoredAuth();
    if (!stored) {
      setLoading(false);
      return;
    }

    apiMe()
      .then(apiUser => {
        setUser({
          id: apiUser.id,
          username: apiUser.username,
          email: apiUser.email,
          defaultCurrency: apiUser.default_currency,
          timezone: apiUser.timezone,
        });
        return loadLedgers();
      })
      .then(mapped => {
        if (mapped.length > 0) {
          setSelectedLedgerId(mapped[0].id);
        }
      })
      .catch(() => {
        clearStoredAuth();
        setTriggerExpNotice(true);
      })
      .finally(() => setLoading(false));
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // ── Reload per-ledger data when selectedLedgerId changes ─────────────────

  useEffect(() => {
    if (!selectedLedgerId || !user) return;
    loadLedgerData(selectedLedgerId).catch(console.error);
  }, [selectedLedgerId, user, loadLedgerData]);

  // ── Auth ──────────────────────────────────────────────────────────────────

  const handleLoginSuccess = async (u: User) => {
    setUser(u);
    const mapped = await loadLedgers();
    if (mapped.length > 0) setSelectedLedgerId(mapped[0].id);
    setView('dashboard');
  };

  const handleLogout = () => {
    clearStoredAuth();
    setUser(null);
    setLedgers([]);
    setTransactions([]);
    setCategories([]);
    setBudgets([]);
    setSelectedLedgerId('');
    setView('auth');
  };

  // ── Ledger CRUD ───────────────────────────────────────────────────────────

  const handleCreateLedger = async (data: { name: string; emoji: string; currency: string; balance: number }) => {
    const result = await apiCreateLedger({
      name: data.name,
      base_currency: data.currency,
      icon: data.emoji,
      initial_balance: data.balance,
    });
    const mapped = mapLedger(result);
    setLedgers(prev => [...prev, mapped]);
    if (!selectedLedgerId) setSelectedLedgerId(mapped.id);
  };

  const handleUpdateLedger = async (id: string, data: { name: string; emoji: string; balance: number }) => {
    const result = await apiUpdateLedger(id, {
      name: data.name,
      icon: data.emoji,
      initial_balance: data.balance,
    });
    setLedgers(prev => prev.map(l => l.id === id ? mapLedger(result) : l));
  };

  const handleDeleteLedger = async (id: string) => {
    await apiDeleteLedger(id);
    setLedgers(prev => {
      const updated = prev.filter(l => l.id !== id);
      if (selectedLedgerId === id && updated.length > 0) {
        setSelectedLedgerId(updated[0].id);
      }
      return updated;
    });
  };

  // ── Transaction CRUD ──────────────────────────────────────────────────────

  const handleAddTransaction = async (newTx: Omit<Transaction, 'id'>) => {
    const ledgerId = selectedLedgerId;
    if (!ledgerId) throw new Error('请先选择账本');

    const cat = categories.find(c => c.name === newTx.category && c.type === newTx.type);

    const result = await apiCreateTransaction(ledgerId, {
      type: newTx.type,
      amount: newTx.amount,
      currency: newTx.currency,
      category_id: cat?.id ?? null,
      note: newTx.note || null,
      occurred_at: new Date(newTx.date + 'T12:00:00.000Z').toISOString(),
    });

    setTransactions(prev => [mapTransaction(result), ...prev]);

    // Refresh ledger balance + budgets from server
    const [updatedLedger, budResp] = await Promise.all([
      apiGetLedger(ledgerId),
      apiGetBudgets(ledgerId),
    ]);
    setLedgers(prev => prev.map(l => l.id === ledgerId ? mapLedger(updatedLedger) : l));
    setBudgets(budResp.map(b => mapBudget(b, categories)));
  };

  const handleDeleteTransaction = async (id: string) => {
    const ledgerId = selectedLedgerId;
    if (!ledgerId) return;

    await apiDeleteTransaction(ledgerId, id);
    setTransactions(prev => prev.filter(t => t.id !== id));

    // Refresh ledger balance + budgets
    const [updatedLedger, budResp] = await Promise.all([
      apiGetLedger(ledgerId),
      apiGetBudgets(ledgerId),
    ]);
    setLedgers(prev => prev.map(l => l.id === ledgerId ? mapLedger(updatedLedger) : l));
    setBudgets(budResp.map(b => mapBudget(b, categories)));
  };

  // ── Category CRUD ─────────────────────────────────────────────────────────

  const handleCreateCategory = async (data: { name: string; type: string; color: string }) => {
    if (!selectedLedgerId) throw new Error('请先选择账本');
    const result = await apiCreateCategory(selectedLedgerId, { name: data.name, type: data.type });
    const mapped: Category = { ...mapCategory(result), color: data.color };
    setCategories(prev => [...prev, mapped]);
  };

  const handleDeleteCategory = async (id: string) => {
    if (!selectedLedgerId) throw new Error('请先选择账本');
    await apiDeleteCategory(selectedLedgerId, id);
    setCategories(prev => prev.filter(c => c.id !== id));
  };

  const handleImportCategories = async (cats: Array<{ name: string; type: string; color: string }>) => {
    if (!selectedLedgerId) throw new Error('请先选择账本');
    const created: Category[] = [];
    for (const c of cats) {
      const result = await apiCreateCategory(selectedLedgerId, { name: c.name, type: c.type });
      created.push({ ...mapCategory(result), color: c.color });
    }
    setCategories(prev => [...prev, ...created]);
  };

  // ── Budget CRUD ───────────────────────────────────────────────────────────

  const handleCreateBudget = async (data: { categoryName?: string; period: string; limitAmount: number }) => {
    if (!selectedLedgerId) throw new Error('请先选择账本');
    const cat = data.categoryName ? categories.find(c => c.name === data.categoryName) : null;
    const result = await apiCreateBudget(selectedLedgerId, {
      category_id: cat?.id ?? null,
      period: uiPeriodToApi(data.period),
      amount: data.limitAmount,
    });
    setBudgets(prev => [...prev, mapBudget(result, categories)]);
  };

  const handleDeleteBudget = async (id: string) => {
    if (!selectedLedgerId) throw new Error('请先选择账本');
    await apiDeleteBudget(selectedLedgerId, id);
    setBudgets(prev => prev.filter(b => b.id !== id));
  };

  // ── Render ────────────────────────────────────────────────────────────────

  const activeLedger = ledgers.find(l => l.id === selectedLedgerId) || ledgers[0];

  if (loading) {
    return (
      <div className="min-h-screen bg-slate-50 flex items-center justify-center">
        <div className="text-slate-400 text-sm font-medium">加载中...</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-slate-50 text-slate-900 font-sans antialiased flex flex-col">
      <div className="flex-1 flex min-h-screen">
        {!user ? (
          <div className="flex-1">
            <Auth onLoginSuccess={handleLoginSuccess} initialExpirationNotice={triggerExpNotice} />
          </div>
        ) : (
          <div className="flex-1 flex">
            <Sidebar
              currentView={currentView}
              setView={setView}
              user={user}
              onLogout={handleLogout}
            />

            <main className="flex-1 bg-slate-50/50 p-6 md:p-10 overflow-y-auto max-h-screen">
              {currentView === 'dashboard' && (
                <Dashboard
                  ledgers={ledgers}
                  transactions={transactions}
                  categories={categories}
                  budgets={budgets}
                  onImportCategories={handleImportCategories}
                  addTransaction={handleAddTransaction}
                  setSelectedLedgerId={setSelectedLedgerId}
                  setView={setView}
                />
              )}

              {currentView === 'ledgers' && (
                <Ledgers
                  ledgers={ledgers}
                  onCreateLedger={handleCreateLedger}
                  onUpdateLedger={handleUpdateLedger}
                  onDeleteLedger={handleDeleteLedger}
                  setSelectedLedgerId={(id) => {
                    setSelectedLedgerId(id);
                  }}
                  setView={setView}
                />
              )}

              {currentView === 'transactions' && activeLedger && (
                <Transactions
                  ledger={activeLedger}
                  transactions={transactions}
                  categories={categories}
                  addTransaction={handleAddTransaction}
                  deleteTransaction={handleDeleteTransaction}
                  setView={setView}
                />
              )}

              {currentView === 'categories' && activeLedger && (
                <Categories
                  ledger={activeLedger}
                  categories={categories}
                  onCreateCategory={handleCreateCategory}
                  onDeleteCategory={handleDeleteCategory}
                  onImportPreset={handleImportCategories}
                  setView={setView}
                />
              )}

              {currentView === 'budgets' && activeLedger && (
                <Budgets
                  ledger={activeLedger}
                  categories={categories}
                  budgets={budgets}
                  onCreateBudget={handleCreateBudget}
                  onDeleteBudget={handleDeleteBudget}
                  setView={setView}
                />
              )}

              {currentView === 'reports' && (
                <Reports
                  ledgers={ledgers}
                  transactions={transactions}
                  categories={categories}
                />
              )}

              {currentView === 'settings' && (
                <Settings
                  user={user}
                  onUpdateUser={setUser}
                />
              )}

              {(currentView === 'transactions' || currentView === 'categories' || currentView === 'budgets') && !activeLedger && (
                <div className="flex items-center justify-center h-64 text-slate-400">
                  <div className="text-center">
                    <p className="font-semibold text-slate-700 mb-2">请先创建或选择一个账本</p>
                    <button onClick={() => setView('ledgers')} className="text-indigo-600 font-bold text-sm cursor-pointer border-none bg-transparent">去创建账本 →</button>
                  </div>
                </div>
              )}
            </main>
          </div>
        )}
      </div>
    </div>
  );
}
