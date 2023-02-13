-- This file should undo anything in `up.sql`

delete from algorithm where crypto in ('sha1', 'sha256', 'sha512')