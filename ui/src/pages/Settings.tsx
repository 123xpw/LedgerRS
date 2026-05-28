/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React, { useState } from 'react';
import { User } from '../types';
import { ShieldAlert, Save, Key, UserCheck } from 'lucide-react';
import { apiUpdateProfile, apiChangePassword } from '../api';

interface SettingsProps {
  user: User;
  onUpdateUser: (user: User) => void;
}

export function Settings({ user, onUpdateUser }: SettingsProps) {
  const [username, setUsername] = useState(user.username);
  const [currency, setCurrency] = useState(user.defaultCurrency);
  const [timezone, setTimezone] = useState(user.timezone);
  const [successInfo, setSuccessInfo] = useState('');
  const [savingInfo, setSavingInfo] = useState(false);

  const [currPassword, setCurrPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [successPassword, setSuccessPassword] = useState('');
  const [errorPassword, setErrorPassword] = useState('');
  const [savingPassword, setSavingPassword] = useState(false);

  const handleSaveInfo = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!username.trim()) return;

    setSavingInfo(true);
    try {
      const updated = await apiUpdateProfile({
        username: username.trim(),
        default_currency: currency,
        timezone: timezone,
      });
      onUpdateUser({
        ...user,
        username: updated.username,
        defaultCurrency: updated.default_currency,
        timezone: updated.timezone,
      });
      setSuccessInfo('基本账户属性保存写入成功！');
      setTimeout(() => setSuccessInfo(''), 3000);
    } catch (err) {
      setSuccessInfo('');
      alert(err instanceof Error ? err.message : '保存失败');
    } finally {
      setSavingInfo(false);
    }
  };

  const handleSavePassword = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!currPassword || !newPassword || !confirmPassword) {
      setErrorPassword('所有字段均为必填项目');
      return;
    }
    if (newPassword.length < 8) {
      setErrorPassword('新密码必须包含至少 8 位有效字元');
      return;
    }
    if (newPassword !== confirmPassword) {
      setErrorPassword('两次输入的新密码不一致，请核对');
      return;
    }

    setSavingPassword(true);
    setErrorPassword('');
    try {
      await apiChangePassword(currPassword, newPassword);
      setSuccessPassword('您的账户登录秘钥修改成功！下次登录时生效。');
      setCurrPassword('');
      setNewPassword('');
      setConfirmPassword('');
      setTimeout(() => setSuccessPassword(''), 4000);
    } catch (err) {
      setErrorPassword(err instanceof Error ? err.message : '密码修改失败');
    } finally {
      setSavingPassword(false);
    }
  };

  return (
    <div className="space-y-6 animate-fade-in">
      <div className="pb-3 border-b border-light">
        <h1 className="text-2xl font-bold text-slate-900 tracking-tight">账户中心与首选项</h1>
        <p className="text-xs text-slate-500 mt-1">
          设置您的默认核算基准币种、系统对应的时区、并调整您的本地保护密码。
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="bg-white border border-slate-200 rounded-xl p-6 shadow-sm flex flex-col justify-between">
          <form onSubmit={handleSaveInfo} className="space-y-4">
            <h3 className="text-sm font-bold text-slate-800 pb-2.5 border-b border-slate-100 flex items-center gap-1.5">
              <UserCheck className="w-4 h-4 text-indigo-500" />
              <span>基本信息与默认首选项</span>
            </h3>

            {successInfo && (
              <div className="p-2.5 bg-emerald-50 border border-emerald-200 text-emerald-800 rounded font-semibold text-xs text-center animate-fade-in">
                {successInfo}
              </div>
            )}

            <div className="form-group space-y-1.5">
              <label className="text-xs font-bold text-slate-500 block">注册邮箱（不可更改）</label>
              <input
                type="email"
                disabled
                value={user.email}
                className="w-full p-2 border border-slate-200 bg-slate-100 rounded-lg text-xs font-bold text-slate-400 cursor-not-allowed outline-none"
              />
            </div>

            <div className="form-group space-y-1.5">
              <label className="text-xs font-bold text-slate-500 block">用户名 / 昵称</label>
              <input
                type="text"
                required
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                className="w-full p-2 border border-slate-200 rounded-lg text-xs bg-slate-50/50 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all outline-none"
              />
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div className="form-group space-y-1.5">
                <label className="text-xs font-bold text-slate-500 block">基准测算金本位币种</label>
                <select
                  value={currency}
                  onChange={(e) => setCurrency(e.target.value)}
                  className="w-full p-2 border border-slate-200 rounded-lg text-xs bg-slate-50/50 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all"
                >
                  <option value="CNY">CNY - 人民币 (默认)</option>
                  <option value="USD">USD - 国际美元</option>
                  <option value="EUR">EUR - 欧洲联盟币</option>
                  <option value="JPY">JPY - 日本日元</option>
                  <option value="GBP">GBP - 大不列颠英镑</option>
                </select>
              </div>

              <div className="form-group space-y-1.5">
                <label className="text-xs font-bold text-slate-500 block">服务同步时区</label>
                <select
                  value={timezone}
                  onChange={(e) => setTimezone(e.target.value)}
                  className="w-full p-2 border border-slate-200 rounded-lg text-xs bg-slate-50/50 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all"
                >
                  <option value="Asia/Shanghai">北京时间 (UTC+8)</option>
                  <option value="Asia/Tokyo">东京时间 (UTC+9)</option>
                  <option value="America/New_York">东部纽约 (UTC-5)</option>
                  <option value="Europe/London">格林威治 (UTC+0)</option>
                </select>
              </div>
            </div>

            <button
              type="submit"
              disabled={savingInfo}
              className="w-full bg-slate-900 hover:bg-black disabled:opacity-60 text-white py-2 rounded-lg text-xs font-bold cursor-pointer transition-all flex items-center justify-center gap-1.5 shadow"
            >
              <Save className="w-3.5 h-3.5" />
              <span>{savingInfo ? '保存中...' : '保存基本选项'}</span>
            </button>
          </form>
        </div>

        <div className="bg-white border border-slate-200 rounded-xl p-6 shadow-sm">
          <form onSubmit={handleSavePassword} className="space-y-4">
            <h3 className="text-sm font-bold text-slate-800 pb-2.5 border-b border-slate-100 flex items-center gap-1.5">
              <Key className="w-4 h-4 text-rose-500" />
              <span>修改本地加密账盘登录秘钥</span>
            </h3>

            {successPassword && (
              <div className="p-2.5 bg-emerald-50 border border-emerald-200 text-emerald-800 rounded font-semibold text-xs text-center animate-fade-in">
                {successPassword}
              </div>
            )}
            {errorPassword && (
              <div className="p-2.5 bg-rose-50 border border-rose-200 text-rose-800 rounded font-semibold text-xs text-center animate-fade-in">
                {errorPassword}
              </div>
            )}

            <div className="form-group space-y-1.5">
              <label className="text-xs font-bold text-slate-500 block">当前密码证明</label>
              <input
                type="password"
                required
                value={currPassword}
                onChange={(e) => setCurrPassword(e.target.value)}
                placeholder="键入当前的登录大锁密码"
                className="w-full p-2 border border-slate-200 rounded-lg text-xs bg-slate-50/50 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all outline-none"
              />
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div className="form-group space-y-1.5">
                <label className="text-xs font-bold text-slate-500 block">设定安全新密码</label>
                <input
                  type="password"
                  required
                  value={newPassword}
                  onChange={(e) => setNewPassword(e.target.value)}
                  placeholder="至少 8 位有效符号"
                  className="w-full p-2 border border-slate-200 rounded-lg text-xs bg-slate-50/50 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all outline-none"
                />
              </div>

              <div className="form-group space-y-1.5">
                <label className="text-xs font-bold text-slate-500 block">再次核对输入</label>
                <input
                  type="password"
                  required
                  value={confirmPassword}
                  onChange={(e) => setConfirmPassword(e.target.value)}
                  placeholder="重复新密码"
                  className="w-full p-2 border border-slate-200 rounded-lg text-xs bg-slate-50/50 focus:bg-white focus:ring-4 focus:ring-indigo-100 focus:border-indigo-600 transition-all outline-none"
                />
              </div>
            </div>

            <button
              type="submit"
              disabled={savingPassword}
              className="w-full bg-rose-600 hover:bg-rose-700 disabled:opacity-60 text-white py-2 rounded-lg text-xs font-bold cursor-pointer transition-all flex items-center justify-center gap-1.5 shadow"
            >
              <ShieldAlert className="w-3.5 h-3.5" />
              <span>{savingPassword ? '保存中...' : '保存修改密钥'}</span>
            </button>
          </form>
        </div>
      </div>
    </div>
  );
}
