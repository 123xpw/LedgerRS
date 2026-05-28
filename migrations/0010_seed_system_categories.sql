-- System preset categories (FR-4.1).
-- ledger_id = NULL means these categories are available in every ledger.
INSERT INTO categories (id, ledger_id, parent_id, name, type, is_system, sort_order) VALUES
    -- ── Expense categories ────────────────────────────────────────────────────
    ('00000000-0000-0000-0001-000000000001', NULL, NULL, '餐饮',   'expense', TRUE,  1),
    ('00000000-0000-0000-0001-000000000002', NULL, NULL, '交通',   'expense', TRUE,  2),
    ('00000000-0000-0000-0001-000000000003', NULL, NULL, '购物',   'expense', TRUE,  3),
    ('00000000-0000-0000-0001-000000000004', NULL, NULL, '居住',   'expense', TRUE,  4),
    ('00000000-0000-0000-0001-000000000005', NULL, NULL, '娱乐',   'expense', TRUE,  5),
    ('00000000-0000-0000-0001-000000000006', NULL, NULL, '医疗',   'expense', TRUE,  6),
    ('00000000-0000-0000-0001-000000000007', NULL, NULL, '教育',   'expense', TRUE,  7),
    ('00000000-0000-0000-0001-000000000008', NULL, NULL, '通讯',   'expense', TRUE,  8),
    ('00000000-0000-0000-0001-000000000009', NULL, NULL, '其他支出','expense', TRUE,  9),
    -- ── Income categories ─────────────────────────────────────────────────────
    ('00000000-0000-0000-0001-000000000010', NULL, NULL, '工资',   'income',  TRUE, 10),
    ('00000000-0000-0000-0001-000000000011', NULL, NULL, '奖金',   'income',  TRUE, 11),
    ('00000000-0000-0000-0001-000000000012', NULL, NULL, '投资',   'income',  TRUE, 12),
    ('00000000-0000-0000-0001-000000000013', NULL, NULL, '其他收入','income',  TRUE, 13)
ON CONFLICT (id) DO NOTHING;
