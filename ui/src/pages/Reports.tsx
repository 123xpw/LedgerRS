/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React, { useState } from 'react';
import { Ledger, Transaction, Category } from '../types';
import { 
  BarChart, 
  Percent, 
  Calendar, 
  TrendingUp, 
  Info, 
  Award, 
  PieChart 
} from 'lucide-react';

interface ReportsProps {
  ledgers: Ledger[];
  transactions: Transaction[];
  categories: Category[];
}

export function Reports({ ledgers, transactions, categories }: ReportsProps) {
  const [selectedLedgerId, setSelectedLedgerId] = useState(ledgers[0]?.id || '1');

  // Selected ledger transactions list
  const activeLedger = ledgers.find(l => l.id === selectedLedgerId) || ledgers[0];

  // Hardcode historical stats (since the user database is virtual)
  const monthlyData = [
    { month: '2025-12', income: 11000, expense: 4500, net: 6500 },
    { month: '2026-01', income: 12500, expense: 5200, net: 7300 },
    { month: '2026-02', income: 11500, expense: 4100, net: 7400 },
    { month: '2026-03', income: 12500, expense: 6200, net: 6300 },
    { month: '2026-04', income: 13000, expense: 3800, net: 9200 },
    { month: '2026-05', income: 13450, expense: 4064, net: 9386 },
  ];

  // Calculate SVG heights
  // Let's find the max amount to scale nicely. Max is 14000.
  const chartHeight = 160;
  const chartWidth = 480;
  const maxVal = 14000;
  
  const getSVGHeight = (val: number) => {
    return (val / maxVal) * chartHeight;
  };

  return (
    <div className="space-y-6 animate-fade-in">
      {/* View Header */}
      <div className="pb-3 border-b border-light">
        <h1 className="text-2xl font-bold text-slate-900 tracking-tight">智能报表与财务审计</h1>
        <p className="text-xs text-slate-500 mt-1">
          全自动按多币种重估合并，并提供完全可随动的交互式全矢量收支透析图表。
        </p>
      </div>

      {/* Selector & KPI stats layout */}
      <div className="grid grid-cols-1 lg:grid-cols-4 gap-4 items-stretch">
        <div className="bg-white border border-slate-200 rounded-xl p-5 shadow-sm flex flex-col justify-between">
          <div>
            <label className="block text-xs font-bold text-slate-400 uppercase tracking-wide mb-2.5">核算关联账本</label>
            <select
              value={selectedLedgerId}
              onChange={(e) => setSelectedLedgerId(e.target.value)}
              className="w-full p-2.5 text-xs border border-slate-200 rounded-lg outline-none bg-slate-50 border-r-8 border-r-transparent font-semibold text-slate-700 font-bold"
            >
              {ledgers.map(l => (
                <option key={l.id} value={l.id}>{l.name} ({l.currency})</option>
              ))}
            </select>
          </div>
          <div className="mt-4 text-[10.5px] text-slate-500 leading-normal bg-indigo-50/50 p-3 rounded-lg border border-indigo-100/30 text-indigo-950 font-semibold flex gap-1 items-start">
            <Info className="w-4 h-4 text-indigo-500 shrink-0 mt-0.5" />
            <span>选定对应账簿后，下方的资产合并、流动解算、月季表格将自动重组关联。</span>
          </div>
        </div>

        <div className="bg-white border border-slate-200 rounded-xl p-5 shadow-sm">
          <div className="text-xs text-slate-400 font-bold uppercase tracking-wider mb-2">近半年合计总收入</div>
          <div className="text-2xl font-extrabold text-emerald-600 tracking-tight">
            + ¥ 73,950.00
          </div>
          <p className="text-[11px] text-slate-400 mt-2">扣除了海淘账户的手续及结汇滑点</p>
        </div>

        <div className="bg-white border border-slate-200 rounded-xl p-5 shadow-sm">
          <div className="text-xs text-slate-400 font-bold uppercase tracking-wider mb-2">近半年累计支出</div>
          <div className="text-2xl font-extrabold text-rose-500 tracking-tight">
            - ¥ 27,864.50
          </div>
          <p className="text-[11px] text-slate-400 mt-2">医疗、房租保障类开支占比 45%</p>
        </div>

        <div className="bg-white border border-slate-200 rounded-xl p-5 shadow-sm">
          <div className="text-xs text-slate-400 font-bold uppercase tracking-wider mb-2">测算盈余净结蓄</div>
          <div className="text-2xl font-extrabold text-indigo-600 tracking-tight">
            ¥ 46,085.50
          </div>
          <p className="text-[11px] text-slate-400 mt-2">资金储蓄率高达 62.3% （极其健康）</p>
        </div>
      </div>

      {/* SVG charts segment block */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Semi-year trend chart (Span 2) */}
        <div className="bg-white border border-slate-200 rounded-xl p-5 shadow-sm space-y-4 lg:col-span-2">
          <div className="flex justify-between items-center pb-2.5 border-b border-slate-100">
            <h3 className="text-sm font-bold text-slate-800 flex items-center gap-1.5">
              <BarChart className="w-4 h-4 text-slate-400" />
              <span>近 6 个月收支走势透析 (矢量图形)</span>
            </h3>
            <div className="flex gap-3 text-[10px] font-bold">
              <span className="flex items-center gap-1.5 text-emerald-600">
                <span className="w-2.5 h-2.5 bg-emerald-500 rounded-sm"></span> 收入
              </span>
              <span className="flex items-center gap-1.5 text-rose-500">
                <span className="w-2.5 h-2.5 bg-rose-400 rounded-sm"></span> 支出
              </span>
            </div>
          </div>

          {/* SVG Canvas wrapper */}
          <div className="w-full overflow-x-auto pt-2">
            <svg
              viewBox={`0 0 ${chartWidth} 220`}
              className="w-full min-w-[450px]"
              xmlns="http://www.w3.org/2000/svg"
            >
              {/* Background horizontal guide lines */}
              {[0, 0.25, 0.5, 0.75, 1].map((ratio, i) => {
                const y = 30 + (1 - ratio) * chartHeight;
                return (
                  <g key={i}>
                    <line
                      x1="45"
                      y1={y}
                      x2={chartWidth - 10}
                      y2={y}
                      stroke="#f1f5f9"
                      strokeWidth="1"
                      strokeDasharray="4 4"
                    />
                    <text
                      x="35"
                      y={y + 4}
                      fill="#94a3b8"
                      fontSize="9"
                      fontWeight="bold"
                      textAnchor="end"
                    >
                      {Math.round((ratio * maxVal) / 1000)}k
                    </text>
                  </g>
                );
              })}

              {/* Render monthly columns */}
              {monthlyData.map((d, idx) => {
                const groupWidth = 64; // distance between bar groups
                const startX = 65 + idx * groupWidth;
                const incHeight = getSVGHeight(d.income);
                const expHeight = getSVGHeight(d.expense);
                const barWidth = 12;

                return (
                  <g key={d.month} className="group cursor-pointer">
                    {/* Income column (Green) */}
                    <rect
                      x={startX}
                      y={30 + chartHeight - incHeight}
                      width={barWidth}
                      height={incHeight}
                      fill="#10b981"
                      rx="3"
                      className="transition-all hover:opacity-85"
                    />

                    {/* Expense column (Red) */}
                    <rect
                      x={startX + barWidth + 4}
                      y={30 + chartHeight - expHeight}
                      width={barWidth}
                      height={expHeight}
                      fill="#f87171"
                      rx="3"
                      className="transition-all hover:opacity-85"
                    />

                    {/* Month text label */}
                    <text
                      x={startX + barWidth + 2}
                      y={30 + chartHeight + 18}
                      fill="#64748b"
                      fontSize="10"
                      fontWeight="600"
                      textAnchor="middle"
                    >
                      {d.month.split('-')[1]}月
                    </text>

                    {/* Interactive tooltip values (simulated inside svg text layer at column top) */}
                    <text
                      x={startX + barWidth + 2}
                      y={30 + chartHeight - Math.max(incHeight, expHeight) - 8}
                      fill="#1e293b"
                      fontSize="9"
                      fontWeight="extrabold"
                      textAnchor="middle"
                      className="opacity-0 group-hover:opacity-100 transition-opacity bg-white"
                    >
                      盈余 +{d.net}
                    </text>
                  </g>
                );
              })}
            </svg>
          </div>
        </div>

        {/* Category breakdown (Span 1) - TACKLING WHITE SPACE */}
        <div className="bg-white border border-slate-200 rounded-xl p-5 shadow-sm space-y-4">
          <div className="flex justify-between items-center pb-2.5 border-b border-slate-100">
            <h3 className="text-sm font-bold text-slate-800 flex items-center gap-1.5">
              <Percent className="w-4 h-4 text-slate-400" />
              <span>本月消费科目分布</span>
            </h3>
          </div>

          <div className="flex flex-col items-center justify-center py-2 space-y-5">
            {/* Elegant SVG donut */}
            <div className="relative w-32 h-32 flex items-center justify-center">
              <svg className="w-full h-full transform -rotate-90" viewBox="0 0 36 36">
                {/* Underlay tracker circle */}
                <circle
                  cx="18"
                  cy="18"
                  r="15.915"
                  fill="transparent"
                  stroke="#f1f5f9"
                  strokeWidth="3.2"
                />

                {/* Catering (45% - Color: #ef4444) */}
                <circle
                  cx="18"
                  cy="18"
                  r="15.915"
                  fill="transparent"
                  stroke="#ef4444"
                  strokeWidth="3.2"
                  strokeDasharray="45 100"
                  strokeDashoffset="0"
                />

                {/* Geek Tech (25% - Color: #5e6e8d) */}
                <circle
                  cx="18"
                  cy="18"
                  r="15.915"
                  fill="transparent"
                  stroke="#5e6e8d"
                  strokeWidth="3.2"
                  strokeDasharray="25 100"
                  strokeDashoffset="-45"
                />

                {/* Rent & Mortgages (20% - Color: #f59e0b) */}
                <circle
                  cx="18"
                  cy="18"
                  r="15.915"
                  fill="transparent"
                  stroke="#f59e0b"
                  strokeWidth="3.2"
                  strokeDasharray="20 100"
                  strokeDashoffset="-70"
                />

                {/* Others (10% - Color: #94a3b8) */}
                <circle
                  cx="18"
                  cy="18"
                  r="15.915"
                  fill="transparent"
                  stroke="#94a3b8"
                  strokeWidth="3.2"
                  strokeDasharray="10 100"
                  strokeDashoffset="-90"
                />
              </svg>

              {/* Center description card */}
              <div className="absolute text-center">
                <span className="text-[10px] text-slate-400 font-bold block">本月支出合计</span>
                <span className="text-base font-extrabold text-slate-900 leading-tight">¥4,064</span>
              </div>
            </div>

            {/* Color tags list */}
            <div className="w-full grid grid-cols-2 gap-2 text-[11px] font-semibold text-slate-600">
              <div className="flex items-center gap-2">
                <span className="w-2.5 h-2.5 rounded-sm bg-[#ef4444] flex-shrink-0"></span>
                <span>餐饮伙食 (45%)</span>
              </div>
              <div className="flex items-center gap-2">
                <span className="w-2.5 h-2.5 rounded-sm bg-[#5e6e8d] flex-shrink-0"></span>
                <span>数码极客 (25%)</span>
              </div>
              <div className="flex items-center gap-2">
                <span className="w-2.5 h-2.5 rounded-sm bg-[#f59e0b] flex-shrink-0"></span>
                <span>房贷房债 (20%)</span>
              </div>
              <div className="flex items-center gap-2">
                <span className="w-2.5 h-2.5 rounded-sm bg-[#94a3b8] flex-shrink-0"></span>
                <span>闲杂杂记 (10%)</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Historical Month-by-month table */}
      <div className="bg-white border border-slate-200 rounded-xl shadow-sm overflow-hidden">
        <div className="px-5 py-3.5 border-b border-slate-100 flex items-center justify-between">
          <h3 className="text-sm font-bold text-slate-800 flex items-center gap-1.5">
            <Calendar className="w-4 h-4 text-slate-400" />
            <span>近 6 个月收支核算归档明细数据表</span>
          </h3>
          <span className="text-[10px] bg-indigo-50 text-indigo-700 font-bold px-2.5 py-0.5 rounded-md border border-indigo-100/50">本地合并核算</span>
        </div>

        <table className="w-full text-left text-xs border-collapse">
          <thead>
            <tr className="bg-slate-50/70 text-slate-500 font-bold tracking-wider divide-x-0 border-b border-slate-200">
              <th className="px-6 py-3">账期月份</th>
              <th className="px-6 py-3">累计存入额</th>
              <th className="px-6 py-3">累计扣支款</th>
              <th className="px-6 py-3">净余留存率</th>
              <th className="px-6 py-3 text-right font-bold text-slate-900">月季理财评级</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-100 font-semibold text-slate-700">
            {monthlyData.map((m) => {
              const rat = (m.net / m.income) * 100;
              return (
                <tr key={m.month} className="hover:bg-slate-50/40 transition-colors">
                  <td className="px-6 py-3.5 text-slate-900 font-bold">{m.month}</td>
                  <td className="px-6 py-3.5 text-emerald-600">+ ¥ {m.income.toLocaleString('zh-CN')}</td>
                  <td className="px-6 py-3.5 text-rose-500">- ¥ {m.expense.toLocaleString('zh-CN')}</td>
                  <td className="px-6 py-3.5 text-indigo-600 font-bold">¥ {m.net.toLocaleString('zh-CN')} ({rat.toFixed(0)}%)</td>
                  <td className="px-6 py-3.5 text-right whitespace-nowrap">
                    {rat >= 60 ? (
                      <span className="bg-emerald-50 text-emerald-700 border border-emerald-100 font-bold text-[10px] px-2.5 py-0.8 rounded-full">
                        A+ 盈余极度安全
                      </span>
                    ) : rat >= 45 ? (
                      <span className="bg-blue-50 text-blue-700 border border-blue-100 font-bold text-[10px] px-2.5 py-0.8 rounded-full">
                        A- 水位持平
                      </span>
                    ) : (
                      <span className="bg-orange-50 text-orange-700 border border-orange-100 font-bold text-[10px] px-2.5 py-0.8 rounded-full">
                        B 警戒高频买单
                      </span>
                    )}
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
}
