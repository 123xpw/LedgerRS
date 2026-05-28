/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React, { useState } from 'react';
import { Copy, Check, Code, FileText, Layout, FileSpreadsheet } from 'lucide-react';

interface CodeFile {
  name: string;
  lang: 'rust' | 'css';
  description: string;
  icon: typeof FileText;
  content: string;
}

const codeFiles: CodeFile[] = [
  {
    name: 'frontend/style.css',
    lang: 'css',
    description: '💡 包含更新后的 Notion/Linear 设计系统，新增的渐变、多列弹性两列/三列网格布局，双栏登录以及侧边栏子路由菜单样式。',
    icon: FileText,
    content: `/* ----------------- LedgerRS Notion & Linear 升级版设计系统 ----------------- */
:root {
  --primary:       #4f46e5;   /* 靛紫，主色 */
  --primary-dark:  #4338ca;
  --primary-light: #f0f0ff;   /* 主色浅底 */
  --primary-ring:  rgba(79, 70, 229, 0.12);
  --danger:        #ef4444;
  --danger-light:  #fff5f5;
  --success:       #16a34a;
  --success-light: #f0fdf4;
  --bg:            #ffffff;   /* 页面背景 */
  --bg-subtle:     #f8fafc;   /* Slate 50 柔和暖底，减少页面高光刺眼 */
  --surface:       #ffffff;   /* 卡片背景 */
  --border:        #e2e8f0;   /* Slate 200 现代边框 */
  --border-light:  #f1f5f9;   /* Slate 100 极其微弱的分隔线 */
  --text:          #0f172a;   /* Slate 900 极深灰，比纯黑更现代 */
  --text-2:        #475569;   /* Slate 600 次要文字 */
  --muted:         #94a3b8;   /* Slate 400 禁用或辅助说明文字 */
  --radius:        10px;      /* 比 8px 更具现代流线感 */
  --radius-sm:     6px;
  --radius-lg:     16px;
  
  /* 精致微阴影组合 */
  --shadow-xs:     0 1px 2px 0 rgba(0, 0, 0, 0.05);
  --shadow:        0 4px 6px -1px rgba(0, 0, 0, 0.05), 0 2px 4px -2px rgba(0, 0, 0, 0.05);
  --shadow-lg:     0 20px 25px -5px rgba(0, 0, 0, 0.08), 0 8px 10px -6px rgba(0, 0, 0, 0.08);
  --sidebar-w:     240px;
}

/* 基础重置与美化 */
body {
  background-color: var(--bg-subtle);
  color: var(--text);
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
}

/* 全局主布局 */
.layout {
  display: flex;
  min-height: 100vh;
}

/* 侧边栏样式 */
.sidebar {
  width: var(--sidebar-w);
  position: sticky;
  top: 0;
  height: 100vh;
  background: var(--surface);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  padding: 20px 16px;
  flex-shrink: 0;
}

.sidebar-logo {
  font-size: 18px;
  font-weight: 700;
  padding: 12px 12px;
  color: var(--text);
  display: flex;
  align-items: center;
  gap: 8px;
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-top: 20px;
}

.sidebar-nav a {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  color: var(--text-2);
  text-decoration: none;
  border-radius: var(--radius-sm);
  font-size: 14px;
  font-weight: 500;
  transition: all 0.15s ease;
}

.sidebar-nav a:hover {
  background: var(--border-light);
  color: var(--text);
}

.sidebar-nav a[aria-current="page"] {
  background: var(--primary-light);
  color: var(--primary);
  font-weight: 600;
}

/* 带有子项的侧边栏抽屉和子路由小标记 */
.sidebar-sub-badge {
  font-size: 11px;
  padding: 2px 6px;
  border-radius: 999px;
  margin-left: auto;
  font-weight: 600;
}

/* 侧边栏贴底用户区 */
.sidebar-user {
  margin-top: auto;
  border-top: 1px solid var(--border-light);
  padding-top: 16px;
}

.sidebar-username {
  font-weight: 600;
  font-size: 14px;
  margin-bottom: 8px;
  padding: 0 12px;
  color: var(--text);
}

/* 主显内容区：限制在大屏最大宽度，保持紧凑和节奏感 */
.main-content {
  flex: 1;
  padding: 32px 40px;
  max-width: 1440px;
  margin: 0 auto;
  width: 100%;
}

/* 页面二级选项卡 Tab 内部导航栏 (Option B: 解决子路由缺失问题的最优 UX) */
.page-tab-nav {
  display: flex;
  gap: 8px;
  border-bottom: 1px solid var(--border);
  margin-bottom: 24px;
  margin-top: -8px; /* 紧贴主标题 */
}

.page-tab-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 16px;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-2);
  border-bottom: 2px solid transparent;
  text-decoration: none;
  transition: all 0.15s ease;
  cursor: pointer;
}

.page-tab-item:hover {
  color: var(--text);
}

.page-tab-item.active {
  color: var(--primary);
  border-bottom-color: var(--primary);
  font-weight: 600;
}

/* 卡片系统 */
.card {
  background: var(--surface);
  border-radius: var(--radius);
  padding: 24px;
  border: 1px solid var(--border);
  box-shadow: var(--shadow-xs);
  transition: box-shadow 0.2s ease, border-color 0.2s ease;
}

.card:hover {
  box-shadow: var(--shadow);
  border-color: #cbd5e1; /* 鼠标悬浮框线略微加深 */
}

/* 三列、两列和四列响应式骨架 */
.grid-1-2 {
  display: grid;
  grid-template-columns: 1fr;
  gap: 20px;
}
@media (min-width: 1024px) {
  .grid-1-2 {
    grid-template-columns: 2fr 1fr; /* 黄金比例：主区占比2，右边常驻小工具占比1（攻克留白多问题） */
  }
}

.grid-2 {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 16px;
}

.grid-3 {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 16px;
}

.grid-4 {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 14px;
}

/* 数据统计 KPI 卡片 */
.stat-card {
  position: relative;
  overflow: hidden;
}

.stat-card .label {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-2);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 6px;
}

.stat-card .value {
  font-size: 26px;
  font-weight: 700;
  letter-spacing: -0.02em;
}

.stat-card .value.income {
  color: var(--success);
}

.stat-card .value.expense {
  color: var(--danger);
}

/* 按钮设计系统 */
button, .btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 14px;
  border-radius: var(--radius-sm);
  font-size: 13.5px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  border: 1px solid transparent;
}

.btn-primary {
  background: var(--primary);
  color: #ffffff;
}
.btn-primary:hover {
  background: var(--primary-dark);
}

.btn-ghost {
  background: transparent;
  border-color: var(--border);
  color: var(--text-2);
}
.btn-ghost:hover {
  background: var(--border-light);
  color: var(--text);
}

.btn-danger {
  background: var(--danger);
  color: #ffffff;
}
.btn-danger:hover {
  opacity: 0.9;
}

/* 双栏登录/注册专用布局 (Option B: 专业高逼格分裂版) */
.auth-split-container {
  display: flex;
  min-height: 100vh;
  background: var(--bg-subtle);
}

.auth-sidebar {
  width: 42%;
  background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
  color: #fff;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 48px;
  position: relative;
  overflow: hidden;
}

@media (max-width: 900px) {
  .auth-sidebar {
    display: none; /* 移动端或窄屏自动降级为单栏 */
  }
}

.auth-form-side {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 40px;
}

/* 表单组合 */
.form-group {
  margin-bottom: 16px;
}
.form-group label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  margin-bottom: 6px;
  color: var(--text-2);
}
.form-group input, .form-group select {
  width: 100%;
  padding: 8px 12px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--border);
  background: var(--surface);
  color: var(--text);
  font-size: 14px;
  outline: none;
  transition: all 0.15s ease;
}
.form-group input:focus, .form-group select:focus {
  border-color: var(--primary);
  box-shadow: 0 0 0 3px var(--primary-ring);
}
`
  },
  {
    name: 'src/pages/auth.rs',
    lang: 'rust',
    description: '💡 全新登录/注册页。左侧为精致宇宙渐变科技面板，带产品亮点及实时标语；右侧是高度聚焦、输入框带软 focus rings 的表单。',
    icon: Layout,
    content: `// auth.rs - 建议修改后的 Leptos 0.7 登录/注册 HTML 结构
use leptos::prelude::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <div class="auth-split-container">
            // 左边栏：高级科技感与品牌故事面板 (Option B)
            <div class="auth-sidebar">
                <div class="sidebar-logo" style="color: #fff; font-size: 22px;">
                    "💰 LedgerRS"
                </div>
                
                <div style="margin-top: auto; margin-bottom: auto; max-width: 360px;">
                    <h2 style="font-size: 28px; font-weight: 700; line-height: 1.3; margin-bottom: 16px; background: linear-gradient(to right, #ffffff, #94a3b8); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">
                        "精细化账本复盘"
                        <br />
                        "掌控每一笔财务脉络"
                    </h2>
                    <p style="color: #94a3b8; font-size: 14px; line-height: 1.6; margin-bottom: 24px;">
                        "采用全新 Notion/Linear 极简风记账体验，配合多币种核算与预算水位预警系统，帮您更快达成财富自由目标。"
                    </p>
                    <div style="display: flex; gap: 12px; align-items: center;">
                        <span class="badge" style="background: rgba(255,255,255,0.08); color: #e2e8f0; font-size: 12px; padding: 4px 10px;">"🔒 军工级本地密钥"</span>
                        <span class="badge" style="background: rgba(255,255,255,0.08); color: #e2e8f0; font-size: 12px; padding: 4px 10px;">"⚡ WASM 疾速响应"</span>
                    </div>
                </div>

                <div style="font-size: 12px; color: #64748b;">
                    "© 2026 LedgerRS Team. 保留一切权利。"
                </div>
            </div>

            // 右边栏：极致细节表单卡片 (高对比，大投影底色)
            <div class="auth-form-side">
                <div class="card" style="width: 100%; max-width: 400px; padding: 36px; border-radius: var(--radius-lg); box-shadow: var(--shadow-lg);">
                    <div style="text-align: center; margin-bottom: 32px;">
                        <h1 style="font-size: 22px; font-weight: 700; margin-bottom: 8px; color: var(--text)">
                            "欢迎回来"
                        </h1>
                        <p style="color: var(--text-2); font-size: 13.5px;">
                            "请登录您的 LedgerRS 个人账户"
                        </p>
                    </div>

                    <form on:submit=move |ev| ev.prevent_default()>
                        <div class="form-group">
                            <label>"邮箱或用户名"</label>
                            <input type="text" placeholder="yourname@example.com" class="form-control" />
                        </div>
                        <div class="form-group">
                            <div style="display: flex; justify-content: space-between; margin-bottom: 6px;">
                                <label style="margin-bottom:0">"账户密码"</label>
                                <a href="#" style="font-size: 12px; color: var(--primary); text-decoration: none;">"忘记密码?"</a>
                            </div>
                            <input type="password" placeholder="••••••••" class="form-control" />
                        </div>

                        // 过期提示或错误通知在这里输出
                        // <div style="font-size:12px; color:var(--danger); margin-bottom:12px;">...</div>

                        <button class="btn btn-primary" style="width: 100%; padding: 10.5px; border-radius: var(--radius); font-weight: 600; margin-top: 10px;">
                            "验证登录"
                        </button>
                    </form>

                    <p style="text-align: center; margin-top: 24px; font-size: 13px; color: var(--text-2)">
                        "还没有账户？"
                        <a href="/register" style="color: var(--primary); font-weight: 600; text-decoration: none;">"立即免费注册"</a>
                    </p>
                </div>
            </div>
        </div>
    }
}
`
  },
  {
    name: 'src/pages/transactions.rs',
    lang: 'rust',
    description: '💡 分类与预算子路由的二级选项卡 UI 结构。解决了用户无导航入口的问题，用户只需要在具体的账本页面点击 Tab 即可无缝穿梭！',
    icon: FileSpreadsheet,
    content: `// transactions.rs (同样适用于 categories.rs 和 budgets.rs)
// Option B: 账本详情顶置页签导航，不污染左侧主目录
use leptos::prelude::*;

#[component]
pub fn TransactionsPage(id: String) -> impl IntoView {
    view! {
        <div>
            // 顶置面包屑与二级页签区
            <div class="page-header" style="margin-bottom: 16px;">
                <div>
                    <div style="display: flex; align-items: center; gap: 8px; font-size: 13px; color: var(--muted); margin-bottom: 4px;">
                        <a href="/ledgers" style="color: var(--muted); text-decoration:none;">"账本管理"</a>
                        <span>"/"</span>
                        <span style="color: var(--text-2)">"当前账本"</span>
                    </div>
                    <h1 class="page-title">"日常活期手账 💰"</h1>
                </div>
                
                <div style="display: flex; gap: 8px; align-items: center;">
                    <button class="btn btn-ghost">"📝 导出 Excel"</button>
                    <button class="btn btn-primary">"+ 新增账目"</button>
                </div>
            </div>

            // 🌟 选项卡导航 - 允许用户在这三个紧密相连的账本子页面自由跳转！
            <div class="page-tab-nav">
                <a href=format!("/ledgers/{}/transactions", id) class="page-tab-item active">
                    "📝 账目记录"
                </a>
                <a href=format!("/ledgers/{}/categories", id) class="page-tab-item">
                    "🏷️ 分类管理"
                </a>
                <a href=format!("/ledgers/{}/budgets", id) class="page-tab-item">
                    "⏱️ 预算控制"
                </a>
            </div>

            // 主展示区（单列 + 左侧过滤，或标准表格）
            <div class="card">
                // ...表格与分页核心代码
            </div>
        </div>
    }
}
`
  },
  {
    name: 'src/pages/dashboard.rs',
    lang: 'rust',
    description: '💡 解决页面空白与无数据引导的设计。多列 1:2 黄金比例网格结构：左侧主看卡片与新增流，右侧常驻快速记账小助手（右侧多列挂载彻底消灭留白）。',
    icon: Layout,
    content: `// dashboard.rs - Leptos 现代多列仪表盘
use leptos::prelude::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div>
            <div class="page-header">
                <div>
                    <h1 class="page-title">"财务总览"</h1>
                    <p style="color: var(--text-2); font-size: 13px; margin-top:2px;">"欢迎回来，胡小明！本月财务状况良好。"</p>
                </div>
                <button class="btn btn-primary">"⚡ 一声快捷记账"</button>
            </div>

            // 核心 1-2 黄金排版网格 (主栏 2fr, 侧栏 1fr)，完美平衡白色空间 (Anti-Whitespace)
            <div class="grid-1-2">
                // 左侧主列
                <div style="display: flex; flex-direction: column; gap: 24px;">
                    // 三列汇总数据卡
                    <div class="grid-3">
                        <div class="card stat-card">
                            <div class="label">"总资产 (本月)"</div>
                            <div class="value">"¥ 24,061.00"</div>
                            <div style="font-size: 11.5px; color: var(--success); margin-top: 6px;">"↑ 较上月稳健增长 4.2%"</div>
                        </div>
                        <div class="card stat-card" style="border-left: 3px solid var(--success);">
                            <div class="label">"本月累计收入"</div>
                            <div class="value income">"+ ¥ 12,950.00"</div>
                        </div>
                        <div class="card stat-card" style="border-left: 3px solid var(--danger);">
                            <div class="label">"本月累计支出"</div>
                            <div class="value expense">"- ¥ 4,064.50"</div>
                        </div>
                    </div>

                    // 我的账本列表
                    <div>
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                            <h2 style="font-size:15px; font-weight:600;">"我的专属账本"</h2>
                            <a href="/ledgers" style="font-size:12.5px; color: var(--primary); text-decoration:none;">"管理全部 →"</a>
                        </div>
                        
                        <div class="grid-3">
                            <a href="/ledgers/1/transactions" style="text-decoration:none; color:inherit;">
                                <div class="card">
                                    <div style="font-size:28px; margin-bottom:8px">"💳"</div>
                                    <div style="font-weight:600; font-size:14.5px;">"日常活期手账"</div>
                                    <div style="color:var(--muted); font-size:12.5px">"币种: CNY"</div>
                                    <div style="font-size:18px; font-weight:700; margin-top:8px; color: var(--text)">"¥ 5,240.50"</div>
                                </div>
                            </a>
                            // ... 其他账本卡片
                        </div>
                    </div>

                    // 🌟 核心点：新用户未配置时的“快速上手/一键引导卡片”（消灭新手登录后的不知所措）
                    <div class="card" style="background: linear-gradient(135deg, var(--primary-light) 0%, #ffffff 100%); border-color: #cbd5e1;">
                        <h3 style="font-weight: 600; margin-bottom: 6px; color: var(--primary)">"🚀 您的 LedgerRS 新手推荐指南"</h3>
                        <p style="font-size:13px; color: var(--text-2); margin-bottom:12px; line-height:1.5;">
                            "我们检测到您的分类较少，导致整体页面偏空。我们已为您准备了多套专属行业记账模版（如：自由职业、萌新大学生、家庭账本），一键即可初始化，赶快去配置吧！"
                        </p>
                        <a href="/ledgers/1/categories" class="btn btn-primary" style="font-size: 12px; padding: 6px 12px;">"立即查看分类模版"</a>
                    </div>
                </div>

                // 右侧侧边栏小部件：放置高密度的常用工具，极大充实页面！
                <div style="display: flex; flex-direction: column; gap: 20px;">
                    // 小部件 1: 瞬时快速记账区
                    <div class="card">
                        <h3 style="font-size:14px; font-weight:600; margin-bottom:12px; border-bottom:1px solid var(--border-light); padding-bottom:8px;">
                            "⚡ 快速记一笔"
                        </h3>
                        <form>
                            <div class="form-group">
                                <label>"分类"</label>
                                <select><option>"日常餐饮"</option><option>"数码极客"</option></select>
                            </div>
                            <div class="form-group">
                                <label>"金额"</label>
                                <input type="number" placeholder="0.00" />
                            </div>
                            <div class="form-group">
                                <label>"备注简短说明"</label>
                                <input type="text" placeholder="比如咖啡或地铁进站" />
                            </div>
                            <button class="btn btn-primary" style="width: 100%; font-size:12px; padding: 8px;">"快速录入"</button>
                        </form>
                    </div>

                    // 小部件 2: 预算水位监视器
                    <div class="card">
                        <h3 style="font-size:14px; font-weight:600; margin-bottom:12px; border-bottom:1px solid var(--border-light); padding-bottom:8px;">
                            "⏱️ 本月预算控制"
                        </h3>
                        <div style="display: flex; flex-direction: column; gap: 12px;">
                            <div>
                                <div style="display:flex; justify-content:space-between; font-size:12.5px; margin-bottom:4px;">
                                    <span>"总支出预算"</span><span>"52.5%"</span>
                                </div>
                                <div style="height:6px; background:var(--border-light); border-radius:3px; overflow:hidden;">
                                    <div style="height:100%; width:52.5%; background:var(--success); border-radius:3px;"></div>
                                </div>
                            </div>
                            // ... 其他预算
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
`
  }
];

export function CodeViewer() {
  const [activeIdx, setActiveIdx] = useState(0);
  const [copied, setCopied] = useState(false);

  const activeFile = codeFiles[activeIdx];

  const handleCopy = () => {
    navigator.clipboard.writeText(activeFile.content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex flex-col h-full bg-slate-900 text-slate-100 rounded-xl overflow-hidden border border-slate-800 shadow-2xl">
      {/* Upper header */}
      <div className="px-6 py-4 border-b border-slate-800 flex items-center justify-between bg-slate-950/80 backdrop-blur">
        <div className="flex items-center gap-2">
          <Code className="w-5 h-5 text-indigo-400" />
          <div>
            <h2 className="text-sm font-semibold text-slate-100">代码与样式拷取看板</h2>
            <p className="text-xs text-slate-400">一键查看并复制最符合 Leptos 0.7 规范的 HTML 与 CSS</p>
          </div>
        </div>
      </div>

      {/* Selector tabs */}
      <div className="flex border-b border-slate-800 bg-slate-900/60 p-1.5 gap-1 overflow-x-auto">
        {codeFiles.map((file, idx) => {
          const Icon = file.icon;
          return (
            <button
              key={file.name}
              onClick={() => {
                setActiveIdx(idx);
                setCopied(false);
              }}
              className={`flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-medium cursor-pointer transition-all whitespace-nowrap ${
                activeIdx === idx
                  ? 'bg-slate-800 text-indigo-400 border border-slate-700/60 shadow-sm'
                  : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/40'
              }`}
            >
              <Icon className="w-3.5 h-3.5" />
              <span>{file.name}</span>
            </button>
          );
        })}
      </div>

      {/* Description */}
      <div className="px-6 py-3 bg-indigo-950/35 border-b border-slate-800 text-xs text-indigo-200/95 leading-relaxed flex items-start gap-2">
        <span className="mt-0.5 bg-indigo-500/20 text-indigo-300 font-bold px-1.5 py-0.2 rounded scale-90">INFO</span>
        <p>{activeFile.description}</p>
      </div>

      {/* Textarea container */}
      <div className="relative flex-1 min-h-[300px] flex flex-col bg-slate-950/90 font-mono text-xs overflow-hidden">
        {/* Copy button */}
        <button
          onClick={handleCopy}
          className="absolute top-4 right-4 z-10 flex items-center gap-1.5 px-3 py-1.5 bg-slate-800/90 border border-slate-700/80 hover:bg-slate-700 hover:text-white rounded-lg cursor-pointer transition-all text-slate-300 select-none shadow"
        >
          {copied ? (
            <>
              <Check className="w-3.5 h-3.5 text-green-400" />
              <span className="text-green-400 font-medium">已复制到剪贴板</span>
            </>
          ) : (
            <>
              <Copy className="w-3.5 h-3.5" />
              <span>复制代码</span>
            </>
          )}
        </button>

        {/* Code representation wrapper */}
        <pre className="flex-1 p-6 overflow-auto scrollbar-thin scrollbar-thumb-slate-800 scrollbar-track-transparent">
          <code className={activeFile.lang === 'css' ? 'text-indigo-200' : 'text-slate-300'}>
            {activeFile.content}
          </code>
        </pre>
      </div>
    </div>
  );
}
