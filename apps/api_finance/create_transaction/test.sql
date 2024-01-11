SELECT
  t.id,
  t.transaction_type_id,
  t.amount,
  t.created_at,
  t.updated_at,
  tg.name 'tag',
  (
    SELECT tt.name
    FROM transaction_types as tt
    WHERE tt.id = t.transaction_type_id
  ) 'type'
FROM transactions as t
LEFT JOIN transactions_tags as tot
ON t.id = tot.transaction_id
LEFT JOIN tags as tg
ON tg.id = tot.tag_id
WHERE t.id = '7a84ffac-d1ab-4373-90c1-4f80e4be0aeb'
