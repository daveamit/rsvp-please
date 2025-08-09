# Rsvp Please! 📨

**Rsvp Please** is an open source project to make casual event planning simple, fast, and actually effective.

Think: chai meetups, momo nights, hiking trips — without the chaos of WhatsApp replies and last-minute flaking.

---

## 🚧 Status: Work in Progress

This project is under active development. Not live yet — but here’s what we’re building.

---

## ✨ Goals

- Create lightweight event pages with RSVP tracking
- No logins, no apps — just share a link
- Simple RSVP responses like “In”, “Out”, “Maybe”
- Real-time updates for hosts
- Clean UI focused on usability, not formality

---

## 🦀 Built with Rust

Why Rust?

- Speed
- Reliability
- Safety
- Because it’s fun, and a bit overkill (just like planning a Goa trip with 6 people)

Planned tech stack:
- Backend: Rust (Actix Web or Axum)
- Frontend: Static HTML/JS (later maybe HTMX or Yew)
- Storage: In-memory for MVP, Redis/Postgres later

---

## 🛣️ Roadmap

### Phase 1 — Core Backend (WIP)
- [x] Tell people before writing a single line of code
- [ ] Rust project setup
- [ ] API to create event
- [ ] RSVP endpoint
- [ ] Basic in-memory event storage

### Phase 2 — Frontend MVP
- [ ] Static event pages
- [ ] RSVP via link (no auth)
- [ ] Display live RSVP status

### Phase 3 — Polish & Features
- [ ] Shareable slugs (e.g. `/event/dosa-night`)
- [ ] Optional RSVP messages
- [ ] Host dashboard

---

## 🤝 Contributing

Want to help?

- If you're into Rust or frontend tinkering, this is a great project to jump into
- Issues will be beginner-friendly where possible
- Code will be open and documented

### To get started:

```bash
git clone https://github.com/daveamit/rsvp-please.git
cd rsvp-please
cargo run

