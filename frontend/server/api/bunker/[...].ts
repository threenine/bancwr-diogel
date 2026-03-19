// Nitro server route to proxy all /api/bunker/* to backend
export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  const backendUrl = config.public.apiBase || 'http://localhost:3000'

  // Get the path after /api/bunker/
  const path = event.context.params?._ || ''
  const url = `${backendUrl}/api/bunker/${path}`

  // Forward the request
  const method = getMethod(event)
  const body = method !== 'GET' ? await readBody(event) : undefined
  const query = getQuery(event)

  try {
    return await $fetch(url, {
      method,
      body,
      query,
      headers: {
        // Forward relevant headers
        'Content-Type': 'application/json',
      },
    })
  } catch (error: any) {
    // Forward error response
    throw createError({
      statusCode: error.response?.status || 500,
      statusMessage: error.response?.statusText || 'Internal Server Error',
      data: error.response?._data,
    })
  }
})
