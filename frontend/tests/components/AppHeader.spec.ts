import { describe, it, expect } from 'vitest'
import { mountSuspended } from '@nuxt/test-utils/runtime'
import AppHeader from '~/components/AppHeader.vue'

describe('AppHeader', () => {
  it('renders navigation links', async () => {
    const component = await mountSuspended(AppHeader)

    expect(component.text()).toContain('Bancwr Diogel')
    expect(component.text()).toContain('Dashboard')
    expect(component.text()).toContain('Config')
    expect(component.text()).toContain('Team')
    expect(component.text()).toContain('Logs')
  })

  it('contains links to correct pages', async () => {
    const component = await mountSuspended(AppHeader)
    const links = component.findAllComponents({ name: 'NuxtLink' })

    const hrefs = links.map(link => link.props('to'))
    expect(hrefs).toContain('/')
    expect(hrefs).toContain('/config')
    expect(hrefs).toContain('/team')
    expect(hrefs).toContain('/logs')
  })
})
