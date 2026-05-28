const BASE = '/api/v1';

// ─── Storage ──────────────────────────────────────────────────────────────────

export interface AuthStorage {
  access_token: string;
  refresh_token: string;
  user: ApiUser;
}

export function getStoredAuth(): AuthStorage | null {
  try {
    const raw = localStorage.getItem('ledger_auth');
    if (!raw) return null;
    return JSON.parse(raw) as AuthStorage;
  } catch {
    return null;
  }
}

export function setStoredAuth(auth: AuthStorage): void {
  localStorage.setItem('ledger_auth', JSON.stringify(auth));
}

export function clearStoredAuth(): void {
  localStorage.removeItem('ledger_auth');
}

function getToken(): string | null {
  return getStoredAuth()?.access_token ?? null;
}

// ─── HTTP Core ────────────────────────────────────────────────────────────────

async function req<T>(
  method: string,
  path: string,
  body?: unknown,
  withAuth = true,
): Promise<T> {
  const headers: Record<string, string> = {};
  if (body !== undefined) headers['Content-Type'] = 'application/json';
  if (withAuth) {
    const token = getToken();
    if (token) headers['Authorization'] = `Bearer ${token}`;
  }

  const resp = await fetch(`${BASE}${path}`, {
    method,
    headers,
    body: body !== undefined ? JSON.stringify(body) : undefined,
  });

  if (resp.status === 401) {
    clearStoredAuth();
    window.location.replace('/');
    throw new Error('未授权，请重新登录');
  }

  if (!resp.ok) {
    const data = await resp.json().catch(() => ({}));
    const msg = (data as Record<string, Record<string, string>>)?.error?.message ?? '请求失败';
    throw new Error(msg);
  }

  return resp.json() as Promise<T>;
}

// ─── API Response Types ────────────────────────────────────────────────────────

export interface ApiUser {
  id: string;
  email: string;
  username: string;
  default_currency: string;
  timezone: string;
  created_at: string;
  updated_at: string;
}

export interface ApiAuthResponse {
  user: ApiUser;
  access_token: string;
  refresh_token: string;
}

export interface ApiLedger {
  id: string;
  name: string;
  icon: string;
  base_currency: string;
  initial_balance: string;
  balance: string;
  this_month_income: string;
  this_month_expense: string;
  budget_usage_rate: number | null;
  created_at: string;
  updated_at: string;
}

export interface ApiTransaction {
  id: string;
  ledger_id: string;
  type: 'income' | 'expense' | 'transfer';
  amount: string;
  currency: string;
  base_amount: string;
  exchange_rate: string;
  category_id: string | null;
  category_name: string | null;
  category_type: string | null;
  note: string | null;
  occurred_at: string;
  created_at: string;
  updated_at: string;
}

export interface ApiPaginated<T> {
  items: T[];
  meta: {
    page: number;
    per_page: number;
    total: number;
    total_pages: number;
  };
}

export interface ApiCategory {
  id: string;
  ledger_id: string | null;
  parent_id: string | null;
  name: string;
  type: string;
  is_system: boolean;
  sort_order: number;
  created_at: string;
}

export interface ApiBudget {
  id: string;
  ledger_id: string;
  category_id: string | null;
  period: string;
  amount: string;
  spent: string;
  remaining: string;
  usage_rate: number;
  period_start: string;
  period_end: string;
  created_at: string;
  updated_at: string;
}

// ─── Auth ──────────────────────────────────────────────────────────────────────

export function apiLogin(credential: string, password: string): Promise<ApiAuthResponse> {
  return req<ApiAuthResponse>('POST', '/auth/login', { credential, password }, false);
}

export function apiRegister(
  email: string,
  username: string,
  password: string,
  timezone?: string,
  default_currency?: string,
): Promise<ApiAuthResponse> {
  return req<ApiAuthResponse>('POST', '/auth/register', { email, username, password, timezone, default_currency }, false);
}

export function apiLogout(refresh_token: string): Promise<void> {
  return req<void>('POST', '/auth/logout', { refresh_token });
}

export function apiMe(): Promise<ApiUser> {
  return req<ApiUser>('GET', '/auth/me');
}

export function apiUpdateProfile(data: { username?: string; default_currency?: string; timezone?: string }): Promise<ApiUser> {
  return req<ApiUser>('PATCH', '/auth/me', data);
}

export function apiChangePassword(current_password: string, new_password: string): Promise<void> {
  return req<void>('PATCH', '/auth/me/password', { current_password, new_password });
}

// ─── Ledgers ──────────────────────────────────────────────────────────────────

export function apiGetLedgers(): Promise<ApiLedger[]> {
  return req<ApiLedger[]>('GET', '/ledgers');
}

export function apiGetLedger(id: string): Promise<ApiLedger> {
  return req<ApiLedger>('GET', `/ledgers/${id}`);
}

export function apiCreateLedger(data: {
  name: string;
  base_currency: string;
  icon?: string;
  initial_balance?: number;
}): Promise<ApiLedger> {
  return req<ApiLedger>('POST', '/ledgers', data);
}

export function apiUpdateLedger(id: string, data: {
  name?: string;
  icon?: string;
  initial_balance?: number;
}): Promise<ApiLedger> {
  return req<ApiLedger>('PATCH', `/ledgers/${id}`, data);
}

export function apiDeleteLedger(id: string): Promise<void> {
  return req<void>('DELETE', `/ledgers/${id}`);
}

// ─── Transactions ─────────────────────────────────────────────────────────────

export function apiGetTransactions(ledgerId: string, page = 1, perPage = 100): Promise<ApiPaginated<ApiTransaction>> {
  return req<ApiPaginated<ApiTransaction>>('GET', `/ledgers/${ledgerId}/transactions?page=${page}&per_page=${perPage}`);
}

export function apiCreateTransaction(ledgerId: string, data: {
  type: string;
  amount: number;
  currency: string;
  category_id?: string | null;
  note?: string | null;
  occurred_at: string;
}): Promise<ApiTransaction> {
  return req<ApiTransaction>('POST', `/ledgers/${ledgerId}/transactions`, data);
}

export function apiDeleteTransaction(ledgerId: string, txId: string): Promise<void> {
  return req<void>('DELETE', `/ledgers/${ledgerId}/transactions/${txId}`);
}

// ─── Categories ───────────────────────────────────────────────────────────────

export function apiGetCategories(ledgerId: string): Promise<ApiCategory[]> {
  return req<ApiCategory[]>('GET', `/ledgers/${ledgerId}/categories`);
}

export function apiCreateCategory(ledgerId: string, data: {
  name: string;
  type: string;
  sort_order?: number;
}): Promise<ApiCategory> {
  return req<ApiCategory>('POST', `/ledgers/${ledgerId}/categories`, data);
}

export function apiDeleteCategory(ledgerId: string, catId: string): Promise<void> {
  return req<void>('DELETE', `/ledgers/${ledgerId}/categories/${catId}`);
}

// ─── Budgets ──────────────────────────────────────────────────────────────────

export function apiGetBudgets(ledgerId: string): Promise<ApiBudget[]> {
  return req<ApiBudget[]>('GET', `/ledgers/${ledgerId}/budgets`);
}

export function apiCreateBudget(ledgerId: string, data: {
  category_id?: string | null;
  period: string;
  amount: number;
}): Promise<ApiBudget> {
  return req<ApiBudget>('POST', `/ledgers/${ledgerId}/budgets`, data);
}

export function apiDeleteBudget(ledgerId: string, budgetId: string): Promise<void> {
  return req<void>('DELETE', `/ledgers/${ledgerId}/budgets/${budgetId}`);
}
