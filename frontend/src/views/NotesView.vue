<template>
  <div class="notes-view">
    <div class="header">
      <h1>Notes</h1>
      <button class="btn btn-primary" @click="showCreateModal = true">+ New Note</button>
    </div>

    <div class="notes-grid" v-if="notes.length > 0">
      <div 
        v-for="note in notes" 
        :key="note.id" 
        class="note-card"
        @click="router.push(`/notes/${note.id}`)"
      >
        <h3>{{ note.title }}</h3>
        <p class="note-description">{{ getDescriptionPreview(note.description) }}</p>
        <div class="note-meta">
          <span>{{ formatDate(note.updated_at) }}</span>
        </div>
      </div>
    </div>

    <div v-else class="empty-state">
      <p>No notes yet. Create your first note!</p>
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
            <button type="button" class="btn btn-secondary" @click="showCreateModal = false">Cancel</button>
            <button type="submit" class="btn btn-primary">Create</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { notesApi } from '../api'
import { formatDate, getDescriptionPreview } from '../composables'
import type { Note } from '../types'
import logger from '@/utils/logger'

const router = useRouter()
const notes = ref<Note[]>([])
const showCreateModal = ref(false)
const newNote = ref({ title: '', description: '' })

const fetchNotes = async () => {
  try {
    const response = await notesApi.getAll()
    notes.value = response.data
  } catch (error) {
    logger.error('Failed to fetch notes:', error)
  }
}

const createNote = async () => {
  try {
    const response = await notesApi.create(newNote.value)
    showCreateModal.value = false
    newNote.value = { title: '', description: '' }
    // Navigate to the new note to add content
    router.push(`/notes/${response.data.id}`)
  } catch (error) {
    logger.error('Failed to create note:', error)
  }
}

onMounted(fetchNotes)
</script>

<style scoped>
.notes-view {
  max-width: 1200px;
  margin: 0 auto;
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
  transition: transform 0.2s, box-shadow 0.2s;
}

.note-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
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
</style>
