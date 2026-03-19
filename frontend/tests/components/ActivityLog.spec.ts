import { describe, it, expect } from 'vitest'
import { mountSuspended } from '@nuxt/test-utils/runtime'
import ActivityLog from '~/components/ActivityLog.vue'

const mockLogs = [
  { id: '1', timestamp: '2025-01-01T10:00:00Z', kind: 1, member: 'Alice', status: 'success' },
  { id: '2', timestamp: '2025-01-01T11:00:00Z', kind: 4, member: 'Bob', status: 'error' }
]

describe('ActivityLog', () => {
  it('renders table from props', async () => {
    const component = await mountSuspended(ActivityLog, {
      props: {
        rows: mockLogs
      }
    })

    expect(component.text()).toContain('Alice')
    expect(component.text()).toContain('Bob')
  })

  it('handles empty state', async () => {
    const component = await mountSuspended(ActivityLog, {
      props: {
        rows: []
      }
    })
    expect(component.text()).toContain('No data')
  })
})
