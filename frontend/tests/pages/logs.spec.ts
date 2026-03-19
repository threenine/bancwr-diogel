import { describe, it, expect, beforeEach } from 'vitest'
import { mountSuspended, registerEndpoint } from '@nuxt/test-utils/runtime'
import Logs from '../../app/pages/logs.vue'
import { clearNuxtData } from '#imports'

const mockLogs = [
  { id: '1', timestamp: '2025-01-01T10:00:00Z', kind: 1, member: 'Alice', status: 'success' }
]

describe('Logs page', () => {
  beforeEach(async () => {
    await clearNuxtData()
  })

  it('renders logs page header', async () => {
    registerEndpoint('/api/bunker/logs', {
      method: 'GET',
      handler: () => mockLogs
    })

    const component = await mountSuspended(Logs)

    expect(component.text()).toContain('Signing Activity Log')
    // The data should be passed to ActivityLog
    expect(component.text()).toContain('Alice')
  })

  it('handles empty logs', async () => {
    registerEndpoint('/api/bunker/logs', {
      method: 'GET',
      handler: () => []
    })

    const component = await mountSuspended(Logs)
    // It should render ActivityLog which shows "No data"
    expect(component.text()).toContain('No data')
  })
})
