DELETE FROM sections
WHERE id=?1
;
DELETE FROM names
WHERE section_id=?1