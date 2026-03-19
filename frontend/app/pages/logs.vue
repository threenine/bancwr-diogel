<script setup lang="ts">
import { useFetch } from '#imports'

const { data: logs } = await useFetch('/api/bunker/logs', { key: 'logs-full' })
</script>

<template>
  <div class="space-y-6">
    <h3 class="text-2xl font-bold">
      Signing Activity Log
    </h3>
    <UCard>
      <ActivityLog :rows="logs" />
    </UCard>
  </div>
</template>
