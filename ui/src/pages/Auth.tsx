/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React, { useState } from 'react';
import { User } from '../types';
import { LegerRSLogo } from '../components/LegerRSLogo';
import { Sparkles, Mail, Lock, ShieldCheck, Zap, ArrowRight } from 'lucide-react';
import { apiLogin, apiRegister, setStoredAuth } from '../api';

interface AuthProps {
  onLoginSuccess: (user: User) => void;
  initialExpirationNotice?: boolean;
}

export function Auth({ onLoginSuccess, initialExpirationNotice = false }: AuthProps) {
  const [isLogin, setIsLogin] = useState(true);
  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [errorMsg, setErrorMsg] = useState('');
  const [loading, setLoading] = useState(false);
  const [showTokenExpired] = useState(initialExpirationNotice);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!password || (!isLogin && (!username || !email))) {
      setErrorMsg('请完整填写表格字段');
      return;
    }
    if (password.length < 6) {
      setErrorMsg('密码至少包含6位字符');
      return;
    }

    setErrorMsg('');
    setLoading(true);

    try {
      let resp;
      if (isLogin) {
        resp = await apiLogin(username, password);
      } else {
        resp = await apiRegister(email, username, password, 'Asia/Shanghai', 'CNY');
      }

      setStoredAuth({
        access_token: resp.access_token,
        refresh_token: resp.refresh_token,
        user: resp.user,
      });

      onLoginSuccess({
        id: resp.user.id,
        username: resp.user.username,
        email: resp.user.email,
        defaultCurrency: resp.user.default_currency,
        timezone: resp.user.timezone,
      });
    } catch (err) {
      setErrorMsg(err instanceof Error ? err.message : '登录失败，请重试');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex min-h-screen bg-slate-50 w-full overflow-hidden">
      {/* LEFT SIDEBAR */}
      <div className="hidden lg:flex w-[42%] bg-gradient-to-br from-slate-50 via-slate-100 to-indigo-50 p-12 flex-col justify-between relative overflow-hidden text-slate-800 border-r border-slate-200/90 shadow-2xs">
        <div className="absolute inset-0 opacity-[0.4] pointer-events-none text-slate-250">
          <svg width="100%" height="100%" xmlns="http://www.w3.org/2000/svg">
            <defs>
              <pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
                <path d="M 40 0 L 0 0 0 40" fill="none" stroke="currentColor" strokeWidth="1" />
              </pattern>
            </defs>
            <rect width="100%" height="100%" fill="url(#grid)" />
          </svg>
        </div>

        <div className="flex items-center gap-3 relative z-10 animate-fade-in">
          <LegerRSLogo size="md" variant="light" />
          <span className="text-[10px] font-bold bg-indigo-100 text-indigo-700 border border-indigo-200/50 px-2 py-0.5 rounded-full ml-1.5">
            v0.7.2
          </span>
        </div>

        <div className="my-auto relative z-10 max-w-[400px]">
          <span className="inline-flex items-center gap-1.5 text-xs font-bold text-indigo-700 bg-indigo-150/40 border border-indigo-200/40 px-2.5 py-1 rounded-full mb-6">
            <Sparkles className="w-3.5 h-3.5 text-indigo-600" />
            <span>全新 Notion/Linear 极简工艺质感</span>
          </span>
          <h2 className="text-2xl font-extrabold tracking-tight leading-tight mb-5 text-slate-900">
            精细化资产分析，
            <br />
            <span className="bg-gradient-to-r from-indigo-700 to-indigo-900 bg-clip-text text-transparent font-black">保障每笔收支的安全。</span>
          </h2>
          <p className="text-slate-500 text-xs leading-relaxed mb-8">
            采用高效率的设计范式与多账簿物理隔离架构，支持账目独立核算、动态多币种转化、灵活支出限额预警及可视化报表。
          </p>

          <div className="flex flex-col gap-3.5">
            <div className="flex items-center gap-3 text-xs text-slate-600 font-medium">
              <div className="w-6 h-6 rounded-lg bg-indigo-100/60 flex items-center justify-center text-indigo-600 border border-indigo-200/40">
                <ShieldCheck className="w-4 h-4" />
              </div>
              <span>多币种活期汇率安全估算</span>
            </div>
            <div className="flex items-center gap-3 text-xs text-slate-600 font-medium">
              <div className="w-6 h-6 rounded-lg bg-indigo-100/60 flex items-center justify-center text-indigo-600 border border-indigo-200/40">
                <Zap className="w-4 h-4" />
              </div>
              <span>极速账单记账与流式分类</span>
            </div>
          </div>
        </div>

        <div className="text-xs text-slate-400 mt-auto relative z-10 flex items-center justify-between font-medium">
          <span>© 2026 LegerRS Team. 保留一切权利。</span>
          <span className="hover:text-slate-600 cursor-pointer transition-colors flex items-center gap-0.5">
            手册指南 <ArrowRight className="w-3 h-3" />
          </span>
        </div>
      </div>

      {/* RIGHT FORM */}
      <div className="flex-1 flex items-center justify-center p-6 md:p-12">
        <div className="w-full max-w-[420px] bg-white border border-slate-200/90 rounded-2xl shadow-xl shadow-slate-100/50 p-8 md:p-10 transition-all">
          {showTokenExpired && (
            <div className="mb-6 p-4 bg-orange-50 border border-orange-200/80 rounded-xl text-orange-800 text-xs leading-relaxed animate-fade-in flex gap-2.5">
              <span className="text-base">⚠️</span>
              <div>
                <p className="font-semibold mb-0.5 text-orange-950">登录信息已过期</p>
                <p className="text-orange-700/90">出于对您财务账单数据的安全保护，您的登录 Token（7天自动轮换）已过期，请重新录入密码进行安全登录。</p>
              </div>
            </div>
          )}

          <div className="text-center mb-8">
            <h1 className="text-2xl font-bold text-slate-900 tracking-tight mb-2">
              {isLogin ? '欢迎回来 LegerRS' : '创建您的金融账本'}
            </h1>
            <p className="text-slate-500 text-xs">
              {isLogin ? '请提供登录凭证，开启无暇分析记账之旅' : '建立独立加密账本账户，保障您的资产流动'}
            </p>
          </div>

          <form onSubmit={handleSubmit} className="space-y-4">
            {errorMsg && (
              <div className="p-3 bg-rose-50 border border-rose-200 text-rose-800 text-xs rounded-lg font-medium">
                {errorMsg}
              </div>
            )}

            {!isLogin && (
              <div className="form-group space-y-1.5">
                <label className="block text-xs font-bold text-slate-700">电子邮箱</label>
                <div className="relative">
                  <Mail className="absolute left-3 top-2.5 w-4 h-4 text-slate-400" />
                  <input
                    type="email"
                    required
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    placeholder="name@example.com"
                    className="w-full pl-9 pr-3.5 py-2 border border-slate-200 rounded-lg text-sm bg-slate-50/50 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all outline-none"
                  />
                </div>
              </div>
            )}

            <div className="form-group space-y-1.5">
              <label className="block text-xs font-bold text-slate-700">
                {isLogin ? '邮箱 / 用户名' : '用户名'}
              </label>
              <div className="relative">
                <Mail className="absolute left-3 top-2.5 w-4 h-4 text-slate-400" />
                <input
                  type="text"
                  required
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                  placeholder={isLogin ? '输入邮箱或用户名' : '输入您在应用中的称呼'}
                  className="w-full pl-9 pr-3.5 py-2 border border-slate-200 rounded-lg text-sm bg-slate-50/50 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all outline-none"
                />
              </div>
            </div>

            <div className="form-group space-y-1.5">
              <div className="flex items-center justify-between">
                <label className="block text-xs font-bold text-slate-700">账户密码</label>
              </div>
              <div className="relative">
                <Lock className="absolute left-3 top-2.5 w-4 h-4 text-slate-400" />
                <input
                  type="password"
                  required
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="••••••••"
                  className="w-full pl-9 pr-3.5 py-2 border border-slate-200 rounded-lg text-sm bg-slate-50/50 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all outline-none"
                />
              </div>
            </div>

            <button
              type="submit"
              disabled={loading}
              className="w-full bg-indigo-600 hover:bg-indigo-700 disabled:opacity-60 text-white font-bold py-2.5 rounded-lg text-sm cursor-pointer transition-all shadow-md shadow-indigo-100 hover:shadow-indigo-200 flex items-center justify-center gap-1.5"
            >
              {loading ? '处理中...' : (isLogin ? '验证并进入账本' : '快速开通账户')}
            </button>
          </form>

          <div className="mt-8 text-center text-xs text-slate-500">
            {isLogin ? (
              <p>
                没有独立账户？{' '}
                <button
                  onClick={() => { setIsLogin(false); setErrorMsg(''); }}
                  className="text-indigo-600 font-bold hover:underline cursor-pointer bg-transparent border-none p-0"
                >
                  立即免费创建
                </button>
              </p>
            ) : (
              <p>
                已有资产账户？{' '}
                <button
                  onClick={() => { setIsLogin(true); setErrorMsg(''); }}
                  className="text-indigo-600 font-bold hover:underline cursor-pointer bg-transparent border-none p-0"
                >
                  去往登录
                </button>
              </p>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
