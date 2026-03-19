## **Bancwr Diogel Web UI — Agent Development Guidelines**

### **Core Philosophy**
- **One step at a time.** Complete the current step fully before moving to the next.
- **YAGNI.** Don't add features not explicitly requested.
- **Server first, client second.** The UI is a thin layer over the bunker API.

### **Project Structure (Nuxt 4)**

Nuxt 4 uses the `app/` directory for application code:

```
web/
├── nuxt.config.ts           # Minimal config with compatibilityVersion: 4
├── package.json
├── pnpm-lock.yaml
├── vitest.config.ts
├── app/                     # Application code (Nuxt 4 default)
│   ├── pages/               # File-based routing
│   │   ├── index.vue        # Dashboard
│   │   ├── config.vue       # Bunker config
│   │   ├── team.vue         # Team management
│   │   └── logs.vue         # Activity logs
│   ├── components/          # Custom components
│   │   ├── AppHeader.vue
│   │   ├── TeamMemberList.vue
│   │   └── ActivityLog.vue
│   ├── composables/         # Shared logic
│   ├── layouts/             # Page layouts (default.vue)
│   └── app.vue              # Root component
├── server/                  # Nitro API routes (root level)
│   └── api/
│       └── bunker/
│           └── [...].ts
└── tests/                   # ALL tests here
    ├── api/
    │    ├── asserts/
    │         ├── bunker 
    │              ├── should_be_ok_with_key_body.js
    │  ├── tests/
    │      ├── bunker
    └── http-client.env.json
    ├── components/
    │   ├── AppHeader.spec.ts
    │   ├── TeamMemberList.spec.ts
    │   └── ActivityLog.spec.ts
    └── pages/
        ├── index.spec.ts
        ├── config.spec.ts
        ├── team.spec.ts
        └── logs.spec.ts
```

**Key Nuxt 4 Changes:**
- Application code lives in `app/` — not root level
- `pages/`, `components/`, `composables/` are inside `app/`
- `server/` stays at root level


### **Nuxt Config**

```typescript
// nuxt.config.ts
export default defineNuxtConfig({
  modules: ['@nuxt/ui'],
  devtools: { enabled: true }
})
```

### **Styling**
- **Nuxt UI v4** (for Nuxt 4 compatibility) for all base components
- **Tailwind CSS** for custom styling when Nuxt UI doesn't cover it
- Responsive by default (mobile-first)

### **Component Usage**

Use Nuxt UI v4 primitives. Examples:

```vue
<UButton color="primary" variant="solid">Submit</UButton>
<UInput v-model="email" type="email" placeholder="Enter email" />
<UTable :rows="members" :columns="columns" />
<UContainer>
  <UVerticalNavigation :links="navLinks" />
</UContainer>
```

### **State Management**
- **No Pinia for MVP.** Use `useState()` for simple shared state.
- Form state: local `ref()` only
- Server data: `useFetch()` or `useAsyncData()`

### **API Layer**
Nitro routes in `server/api/` proxy to bunker (stays at root, not in `app/`).

### **Testing Strategy**
- **All tests in `/tests` folder** — not in `app/` directories
- **Vitest** as runner with `@vue/test-utils`
- Import components from `~/components/Component.vue` in tests

**Test import example:**
```typescript
import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import Component from '~/components/Component.vue'  // Nuxt 4 alias
```

### **Dependencies**
- **Required:** `nuxt` (v4.x), `@nuxt/ui` (v3.x for Nuxt 4), `vue`
- **Testing:** `@vue/test-utils`, `vitest`, `@nuxt/test-utils`
- **Ask first:** Anything outside this list

### **What NOT To Do**
- ❌ Don't put `pages/`, `components/` at root level — use `app/` directory
- ❌ Don't build custom buttons/inputs — use Nuxt UI v3
- ❌ Don't add Pinia for MVP
- ❌ Don't put tests in `app/` folders — use root `/tests`
- ❌ Don't put server routes in `app/server/` — use root `server/`

### **Before Declaring Done**
- [ ] Application code is in `app/` directory
- [ ] `pnpm dev` starts without warnings
- [ ] `pnpm test` passes
- [ ] `pnpm build` completes successfully
