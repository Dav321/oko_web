SELECT sections.id, sections.name, names.id, names.name
FROM sections
LEFT JOIN names on sections.id = names.section_id
ORDER BY sections.name DESC