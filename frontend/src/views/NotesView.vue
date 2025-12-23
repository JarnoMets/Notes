<template>
  <div class="notes-view">
    <div class="header">
      <h1>Notes</h1>
      <button class="btn btn-primary" @click="showCreateModal = true">+ New Note</button>
    </div>
  <div class="notes-view">
    <div class="header">
      <h1>Notes</h1>
      <button class="btn btn-primary" @click="showCreateModal = true">+ New Note</button>
    </div>

    <div v-if="loading" class="notes-grid">
      <div v-for="n in 6" :key="n" class="note-card skeleton">
        <div class="skeleton-title"></div>
        <div class="skeleton-description"></div>
        <div class="skeleton-meta"></div>
      </div>
    </div>

    <div v-else-if="notes.length > 0" class="notes-container">
      <div v-else class="notes-grid">
        <div 
          v-for="note in notes" 
          :key="note.id" 
          class="note-card"
          @click="$router.push(`/notes/${note.id}`)"
        >
          <h3>{{ note.title }}</h3>
          <p class="note-description">{{ getDescriptionPreview(note.description) }}</p>
          <div class="note-meta">
            <span>{{ formatDate(note.updated_at) }}</span>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="empty-state">
      <p>No notes yet. Create your first note!</p>
    </div>

    <!-- Pagination -->
    <div v-if="totalPages > 1" class="pagination">
      <button 
        class="btn btn-secondary" 
        :disabled="currentPage === 1"
        @click="currentPage--"
      >
        Previous
      </button>
      <span class="page-info">
        Page {{ currentPage }} of {{ totalPages }}
      </span>
      <button 
        class="btn btn-secondary" 
        :disabled="currentPage === totalPages"
        @click="currentPage++"
      >
        Next
      </button>
    </div>

    <!-- Create Note Modal -->
    <div v-if="showCreateModal" class="modal-overlay" @click.self="showCreateModal = false">
      <div class="modal">
        <div class="modal-header">
          <h2>New Note</h2>
          <button class="modal-close" @click="showCreateModal = false">&times;</button>
        </div>
        <form @submit.prevent="createNote">
          <div class="form-group">
            <label for="title">Title</label>
            <input 
              id="title" 
              v-model="newNote.title" 
              type="text" 
              required 
              placeholder="Note title"
            />
          </div>
          <div class="form-group">
            <label for="description">Description</label>
            <textarea 
              id="description" 
              v-model="newNote.description" 
              rows="3" 
              placeholder="Brief description of the note..."
            ></textarea>
          </div>
          <div class="modal-actions">
            <button type="button" class="btn btn-secondary" @click="showCreateModal = false" :disabled="creating">Cancel</button>
            <button type="submit" class="btn btn-primary" :disabled="creating">
              <span v-if="creating" class="spinner"></span>
              {{ creating ? 'Creating...' : 'Create' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import { notesApi } from '../api'
import { formatDate, getDescriptionPreview } from '../composables'
import type { Note } from '../types'
import logger from '@/utils/logger'

const allNotes = ref<Note[]>([])
const showCreateModal = ref(false)
const newNote = ref({ title: '', description: '' })
const loading = ref(true)
const creating = ref(false)
const currentPage = ref(1)
const pageSize = ref(20)

const router = useRouter()

const notes = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value
  const end = start + pageSize.value
  return allNotes.value.slice(start, end)
})

const totalPages = computed(() => Math.ceil(allNotes.value.length / pageSize.value))

const fetchNotes = async () => {
  try {
    loading.value = true
    const response = await notesApi.getAll()
    allNotes.value = response.data
  } catch (error) {
    logger.error('Failed to fetch notes:', error)
  } finally {
    loading.value = false
  }
}

const createNote = async () => {
  try {
    creating.value = true
    const response = await notesApi.create(newNote.value)
    showCreateModal.value = false
    newNote.value = { title: '', description: '' }
    // Navigate to the new note to add content
    router.push(`/notes/${response.data.id}`)
  } catch (error) {
    logger.error('Failed to create note:', error)
  } finally {
    creating.value = false
  }
}

onMounted(fetchNotes)
</script>

<style scoped>
.notes-container {
  width: 100%;
}

.notes-virtual-list {
  padding: 1rem 0;
}

.notes-virtual-list :deep(.virtual-list-item) {
  padding: 0 1rem;
  margin-bottom: 1rem;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.notes-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1.5rem;
}

.note-card {
  background: white;
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  cursor: pointer;
  transition: all 0.3s ease;
  animation: fadeIn 0.3s ease-out;
}

.note-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0,0,0,0.15);
}

.note-card h3 {
  margin-bottom: 0.5rem;
  color: #2c3e50;
}

.note-description {
  color: #666;
  font-size: 0.9rem;
  margin-bottom: 1rem;
  line-height: 1.5;
}

.note-meta {
  font-size: 0.8rem;
  color: #999;
}

.empty-state {
  text-align: center;
  padding: 4rem;
  color: #666;
}

/* Pagination */
.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 1rem;
  margin-top: 2rem;
  padding: 1rem;
}

.page-info {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

/* Spinner */
.spinner {
  display: inline-block;
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-radius: 50%;
  border-top-color: white;
  animation: spin 1s ease-in-out infinite;
  margin-right: 8px;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* Skeleton loading */
.skeleton {
  background: var(--bg-primary);
  animation: pulse 1.5s ease-in-out infinite;
}

.skeleton-title {
  height: 1.5rem;
  background: var(--bg-tertiary);
  border-radius: 4px;
  margin-bottom: 0.5rem;
}

.skeleton-description {
  height: 1rem;
  background: var(--bg-tertiary);
  border-radius: 4px;
  margin-bottom: 1rem;
  width: 80%;
}

.skeleton-meta {
  height: 0.8rem;
  background: var(--bg-tertiary);
  border-radius: 4px;
  width: 40%;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.6;
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
