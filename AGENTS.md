# Agent Guidelines for this Repo

Scope: These instructions apply to the entire repository.

Devlog policy
- One file per calendar day under `devlog/`.
- Filename format: `YYYY-MM-DD-<slug>.md` (lowercase, hyphenated slug).
- If you work multiple times in the same day, append to the same file rather than creating a new one.
- Suggested sections (use what fits): Context, Actions taken, Decisions, Troubleshooting, Commands, Next steps.
- Keep entries concise, actionable, and reproducible. Do not include secrets or tokens.
- Timezone: use the developer’s local timezone; if ambiguous, default to UTC.

Style & changes
- Keep changes minimal and focused on the task.
- Update documentation alongside code changes when behavior or setup changes.
- Follow existing naming/structure; avoid gratuitous refactors.

Bevy runtime note
- Audio is disabled by default via `AudioPlugin` being disabled. Keep it that way unless explicitly requested.

