<script setup lang="ts">
import { useFetch, computed } from '#imports'

const props = defineProps({
  rows: {
    type: Array,
    default: () => []
  }
})

const { data: logs } = await useFetch('/api/bunker/logs', {
  immediate: !props.rows
})

const displayLogs = computed(() => props.rows || logs.value || [])

const formatDate = (date) => {
  return new Date(date).toLocaleString()
}
</script>

<template>
  <UTable
    :data="displayLogs"
    :columns="[
      { accessorKey: 'timestamp', header: 'Timestamp' },
      { accessorKey: 'kind', header: 'Event Kind' },
      { accessorKey: 'member', header: 'Member' },
      { accessorKey: 'status', header: 'Status' }
    ]"
  >
    <template #timestamp-cell="{ row }">
      {{ formatDate(row.timestamp) }}
    </template>
    <template #status-cell="{ row }">
      <UBadge
        :color="row.status === 'success' ? 'success' : 'error'"
        variant="subtle"
      >
        <span class="status-text">{{ row.status }}</span>
      </UBadge>
    </template>
  </UTable>
</template>
