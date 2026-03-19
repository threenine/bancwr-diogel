import { describe, it, expect } from 'vitest'
import { mountSuspended } from '@nuxt/test-utils/runtime'
import TeamMemberList from '~/components/TeamMemberList.vue'

const mockTeam = [
  { id: '1', name: 'Alice', pubkey: 'npub1', role: 'admin' },
  { id: '2', name: 'Bob', pubkey: 'npub2', role: 'signer' }
]

describe('TeamMemberList', () => {
  it('renders list from props', async () => {
    const component = await mountSuspended(TeamMemberList, {
      props: {
        data: mockTeam
      }
    })

    expect(component.text()).toContain('Alice')
    expect(component.text()).toContain('Bob')
    expect(component.text()).toContain('admin')
    expect(component.text()).toContain('signer')
  })

  it('handles empty state', async () => {
    const component = await mountSuspended(TeamMemberList, {
      props: {
        data: []
      }
    })
    expect(component.text()).toContain('No data')
  })
})
