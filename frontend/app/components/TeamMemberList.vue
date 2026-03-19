<script setup lang="ts">
import { useFetch, computed } from '#imports'

const props = defineProps<{
  data: any[]
}>()

const emit = defineEmits(['refresh'])

const removeMember = async (id: string) => {
  if (confirm('Are you sure you want to remove this member?')) {
    await $fetch(`/api/bunker/team/${id}`, { method: 'DELETE' })
    emit('refresh')
  }
}
</script>

<template>
  <div class="space-y-4">
    <UTable
      :data="data"
      :columns="[
        { accessorKey: 'name', header: 'Name' },
        { accessorKey: 'pubkey', header: 'Nostr Pubkey' },
        { accessorKey: 'role', header: 'Role' },
        { accessorKey: 'actions', header: 'Actions' }
      ]"
    >
      <template #actions-cell="{ row }">
        <UButton
          color="error"
          variant="ghost"
          icon="i-heroicons-trash"
          @click="removeMember(row.id)"
        />
      </template>
    </UTable>
  </div>
</template>
