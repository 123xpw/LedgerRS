/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

export interface User {
  id: string;
  username: string;
  email: string;
  defaultCurrency: string;
  timezone: string;
}

export interface Ledger {
  id: string;
  name: string;
  emoji: string;
  currency: string;
  balance: number;
  thisMonthIncome?: number;
  thisMonthExpense?: number;
}

export interface Transaction {
  id: string;
  date: string;
  type: 'expense' | 'income' | 'transfer';
  amount: number;
  currency: string;
  category: string;
  note: string;
}

export interface Category {
  id: string;
  name: string;
  type: 'expense' | 'income';
  color: string;
  icon?: string;
}

export interface Budget {
  id: string;
  name: string;
  category?: string;
  period: 'monthly' | 'weekly' | 'yearly';
  limitAmount: number;
  spentAmount: number;
}

export type ViewType =
  | 'auth'
  | 'dashboard'
  | 'ledgers'
  | 'transactions'
  | 'categories'
  | 'budgets'
  | 'reports'
  | 'settings';
